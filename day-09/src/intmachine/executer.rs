use std::collections::HashMap;

use super::{MachineError, MachineInterface, MachineResult, OpCode};

pub struct Executer {
    number: i128,
    program: Vec<i128>,
    counter: i128,
    relative: i128,
    finished: bool,
    interface: Box<dyn MachineInterface>,
    dynamic_memory: HashMap<i128, i128>,
    debug: bool,
}

impl Executer {
    pub fn new(
        number: i128,
        program: &[i128],
        interface: Box<dyn MachineInterface>,
        debug: bool,
    ) -> Self {
        Executer {
            number,
            program: program.to_owned(),
            counter: 0,
            relative: 0,
            finished: false,
            interface,
            dynamic_memory: HashMap::new(),
            debug,
        }
    }

    pub fn get(&self, index: i128) -> MachineResult<i128> {
        let value = if 0 <= index && index < self.program.len() as i128 {
            Some(self.program[index as usize])
        } else if self.dynamic_memory.contains_key(&index) {
            Some(self.dynamic_memory[&index])
        } else {
            Some(0)
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

    fn set(&mut self, index: i128, value: i128) -> MachineResult<()> {
        let value = if 0 <= index && index < self.program.len() as i128 {
            self.program[index as usize] = value;
            Some(())
        } else {
            self.dynamic_memory.insert(index, value);
            Some(())
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

    fn param(&self, position: i128, code: &OpCode) -> MachineResult<i128> {
        let value = self.get(self.counter + position)?;

        match code.mode(position) {
            0 => self.get(value),
            1 => Ok(value),
            2 => self.get(value + self.relative),
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

    fn set_param(&mut self, position: i128, code: &OpCode, value: i128) -> MachineResult<()> {
        let v = self.get(self.counter + position)?;

        match code.mode(position) {
            0 => self.set(v, value),
            2 => self.set(v + self.relative, value),
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

                let value = self.interface.receive()?;

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

                self.interface.send(param_1)?;

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

                let value = (param_1 < param_2) as i128;
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

                let value = (param_1 == param_2) as i128;
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
            9 => {
                // set relative
                let param_1 = self.param(1, &code)?;

                self.relative += param_1;
                if self.debug {
                    println!(
                        "Executer[{}]: Op: {:?} | relative = {}",
                        self.number,
                        &self.program[(self.counter as usize)..(self.counter as usize + 2)],
                        param_1
                    );
                }

                self.counter += 2;
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
