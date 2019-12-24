use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

use super::{AllocationMode, MachineError, MachineResult, OpCode};

#[derive(Debug)]
pub struct Executer {
    number: i32,
    program: Vec<i32>,
    counter: i32,
    finished: bool,
    in_channel: Receiver<i32>,
    out_channel: Sender<i32>,
    dynamic_memory: HashMap<i32, i32>,
    allocation_mode: AllocationMode,
    debug: bool,
}

impl Executer {
    pub fn new(
        number: i32,
        program: &[i32],
        in_channel: Receiver<i32>,
        out_channel: Sender<i32>,
        allocation_mode: AllocationMode,
        debug: bool,
    ) -> Self {
        Executer {
            number,
            program: program.to_owned(),
            counter: 0,
            finished: false,
            in_channel,
            out_channel,
            dynamic_memory: HashMap::new(),
            allocation_mode,
            debug,
        }
    }

    pub fn get(&self, index: i32) -> MachineResult<i32> {
        let value = if 0 <= index && index < self.program.len() as i32 {
            Some(self.program[index as usize])
        } else {
            match self.allocation_mode {
                AllocationMode::Forbidden => None,
                AllocationMode::AtWrite => {
                    if self.dynamic_memory.contains_key(&index) {
                        Some(self.dynamic_memory[&index])
                    } else {
                        None
                    }
                }
                AllocationMode::DefaultTo(default) => {
                    if self.dynamic_memory.contains_key(&index) {
                        Some(self.dynamic_memory[&index])
                    } else {
                        Some(default)
                    }
                }
            }
        };

        match value {
            Some(v) => Ok(v),
            None => Err(MachineError {
                message: "Index out of bounce!".to_owned(),
                reason: format!(
                    "Try to read element {}, but range is 0 <= x < {}.",
                    index,
                    self.program.len()
                ),
            }),
        }
    }

    fn set(&mut self, index: i32, value: i32) -> MachineResult<()> {
        let value = if 0 <= index && index < self.program.len() as i32 {
            self.program[index as usize] = value;
            Some(())
        } else {
            match self.allocation_mode {
                AllocationMode::Forbidden => None,
                AllocationMode::AtWrite | AllocationMode::DefaultTo(_) => {
                    self.dynamic_memory.insert(index, value);
                    Some(())
                }
            }
        };

        match value {
            Some(_) => Ok(()),
            None => Err(MachineError {
                message: "Index out of bounce!".to_owned(),
                reason: format!(
                    "Try to write element {}, but range is 0 <= x < {}.",
                    index,
                    self.program.len()
                ),
            }),
        }
    }

    fn param(&self, position: i32, code: &OpCode) -> MachineResult<i32> {
        let value = self.get(self.counter + position)?;

        match code.mode(position) {
            0 => self.get(value),
            1 => Ok(value),
            _ => Err(MachineError {
                message: "Illegal parameter mode!".to_owned(),
                reason: format!(
                    "Found parameter mode '{}' at index {}.",
                    code.mode(position),
                    self.counter + position
                ),
            }),
        }
    }

    fn set_param(&mut self, position: i32, code: &OpCode, value: i32) -> MachineResult<()> {
        let v = self.get(self.counter + position)?;

        match code.mode(position) {
            0 => self.set(v, value),
            _ => Err(MachineError {
                message: "Illegal parameter mode!".to_owned(),
                reason: format!(
                    "Found parameter mode '{}' at index {}.",
                    code.mode(position),
                    self.counter + position
                ),
            }),
        }
    }

    pub fn run(&mut self) -> bool {
        while !self.finished {
            let step_result = self.perform_step();

            if let Err(error) = step_result {
                let index = self.counter as usize;
                println!("Executer[{}]: Exception at index {}!", self.number, index);
                let slice = &self.program[index..index + 4];
                println!("Executer[{}]: Program: {:?}", self.number, slice);
                println!("Executer[{}]: {:?}", self.number, error);

                return false;
            }
        }

        true
    }

    fn perform_step(&mut self) -> MachineResult<()> {
        let code = OpCode::new(self.get(self.counter)?);

        match code.op() {
            1 => {
                // Add
                let param_1 = self.param(1, &code)?;
                let param_2 = self.param(2, &code)?;

                let value = param_1 + param_2;
                self.set_param(3, &code, value)?;

                if self.debug {
                    println!(
                        "Executer[{}]: Op: {:?} | {} + {} = {}",
                        self.number,
                        &self.program[(self.counter as usize)..(self.counter as usize + 4)],
                        param_1,
                        param_2,
                        value
                    );
                }

                self.counter += 4;
            }
            2 => {
                // Multiply
                let param_1 = self.param(1, &code)?;
                let param_2 = self.param(2, &code)?;

                let value = param_1 * param_2;
                self.set_param(3, &code, value)?;

                if self.debug {
                    println!(
                        "Executer[{}]: Op: {:?} | {} * {} = {}",
                        self.number,
                        &self.program[(self.counter as usize)..(self.counter as usize + 4)],
                        param_1,
                        param_2,
                        value
                    );
                }

                self.counter += 4;
            }
            3 => {
                // Input

                if self.debug {
                    println!(
                        "Executer[{}]: Op: {:?} | input",
                        self.number,
                        &self.program[(self.counter as usize)..(self.counter as usize + 2)]
                    );
                }

                let value = self.in_channel.recv()?;

                if self.debug {
                    println!(
                        "Executer[{}]: Op: {:?} | input -> done {}",
                        self.number,
                        &self.program[(self.counter as usize)..(self.counter as usize + 2)],
                        value
                    );
                }

                self.set_param(1, &code, value)?;

                self.counter += 2;
            }
            4 => {
                // Output
                let param_1 = self.param(1, &code)?;

                if self.debug {
                    println!(
                        "Executer[{}]: Op: {:?} | output {}",
                        self.number,
                        &self.program[(self.counter as usize)..(self.counter as usize + 2)],
                        param_1
                    );
                }

                self.out_channel.send(param_1)?;

                if self.debug {
                    println!(
                        "Executer[{}]: Op: {:?} | output {} -> done",
                        self.number,
                        &self.program[(self.counter as usize)..(self.counter as usize + 2)],
                        param_1
                    );
                }

                self.counter += 2;
            }
            5 => {
                // jump if true
                let param_1 = self.param(1, &code)?;
                let param_2 = self.param(2, &code)?;

                if self.debug {
                    println!(
                        "Executer[{}]: Op: {:?} | jump if {} != 0",
                        self.number,
                        &self.program[(self.counter as usize)..(self.counter as usize + 3)],
                        param_1
                    );
                }

                if param_1 != 0 {
                    self.counter = param_2;
                } else {
                    self.counter += 3;
                }
            }
            6 => {
                // jump if false
                let param_1 = self.param(1, &code)?;
                let param_2 = self.param(2, &code)?;

                if self.debug {
                    println!(
                        "Executer[{}]: Op: {:?} | jump if {} == 0",
                        self.number,
                        &self.program[(self.counter as usize)..(self.counter as usize + 3)],
                        param_1
                    );
                }

                if param_1 == 0 {
                    self.counter = param_2;
                } else {
                    self.counter += 3;
                }
            }
            7 => {
                // less than
                let param_1 = self.param(1, &code)?;
                let param_2 = self.param(2, &code)?;

                let value = (param_1 < param_2) as i32;
                self.set_param(3, &code, value)?;

                if self.debug {
                    println!(
                        "Executer[{}]: Op: {:?} | {} < {} = {}",
                        self.number,
                        &self.program[(self.counter as usize)..(self.counter as usize + 4)],
                        param_1,
                        param_2,
                        value
                    );
                }

                self.counter += 4;
            }
            8 => {
                // equals than
                let param_1 = self.param(1, &code)?;
                let param_2 = self.param(2, &code)?;

                let value = (param_1 == param_2) as i32;
                self.set_param(3, &code, value)?;

                if self.debug {
                    println!(
                        "Executer[{}]: Op: {:?} | {} == {} = {}",
                        self.number,
                        &self.program[(self.counter as usize)..(self.counter as usize + 4)],
                        param_1,
                        param_2,
                        value
                    );
                }

                self.counter += 4;
            }
            99 => {
                if self.debug {
                    println!(
                        "Executer[{}]: Op: {:?} | exit",
                        self.number,
                        &self.program[(self.counter as usize)..=(self.counter as usize)]
                    );
                }

                self.finished = true;
            }
            _ => {
                return Err(MachineError {
                    message: "Illegal OpCode!".to_owned(),
                    reason: format!("Found OpCode '{}' at index {}.", code.op(), self.counter),
                });
            }
        };

        Ok(())
    }
}
