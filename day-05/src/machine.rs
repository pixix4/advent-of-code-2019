use std::io::{self, Write};

#[derive(Debug)]
pub struct MachineError {
    message: String,
}

impl From<std::io::Error> for MachineError {
    fn from(_error: std::io::Error) -> Self {
        MachineError {
            message: "I/O error!".to_owned(),
        }
    }
}

#[must_use]
pub type MachineResult<T> = Result<T, MachineError>;

#[derive(Debug)]
struct OpCode {
    code: i32,
}

impl OpCode {
    pub fn new(code: i32) -> Self {
        OpCode { code }
    }

    pub fn op(&self) -> i32 {
        self.code % 100
    }

    pub fn mode(&self, pos: i32) -> i32 {
        let position = (10 as i32).pow(pos as u32 + 1);
        (self.code / position % 10)
    }
}

#[derive(Debug)]
pub struct Machine {
    program: Vec<i32>,
    counter: i32,
    finished: bool,
}

impl Machine {
    pub fn new(program: &[i32]) -> Self {
        Machine {
            program: program.to_owned(),
            counter: 0,
            finished: false,
        }
    }

    pub fn get(&self, index: i32) -> MachineResult<i32> {
        match self.program.get(index as usize) {
            Some(v) => Ok(*v),
            None => Err(MachineError {
                message: format!(
                    "Index out of bounce! Try to access element {}, but range is 0 <= x < {}.",
                    index,
                    self.program.len()
                ),
            }),
        }
    }

    fn set(&mut self, index: i32, value: i32) -> MachineResult<()> {
        if 0 <= index && index < self.program.len() as i32 {
            self.program[index as usize] = value;
            Ok(())
        } else {
            Err(MachineError {
                message: format!(
                    "Index out of bounce! Try to access element {}, but range is 0 <= x < {}.",
                    index,
                    self.program.len()
                ),
            })
        }
    }

    fn param(&self, position: i32, code: &OpCode) -> MachineResult<i32> {
        let value = self.get(self.counter + position)?;

        match code.mode(position) {
            0 => self.get(value),
            1 => Ok(value),
            _ => Err(MachineError {
                message: format!(
                    "Illegal parameter mode! Found parameter mode '{}' at index {}.",
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
                message: format!(
                    "Illegal parameter mode! Found parameter mode '{}' at index {}.",
                    code.mode(position),
                    self.counter + position
                ),
            }),
        }
    }

    pub fn _run_with_params(&mut self, params: &[i32]) -> bool {
        for (index, param) in params.iter().enumerate() {
            self.program[index + 1] = *param;
        }

        self.run()
    }

    pub fn run(&mut self) -> bool {
        while !self.finished {
            let step_result = self.perform_step();

            if let Err(error) = step_result {
                let index = self.counter as usize;
                eprintln!("Exception at index {}!", index);
                let slice = &self.program[index..index + 4];
                eprintln!("Program: {:?}", slice);
                eprintln!("{:?}", error);
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

                self.counter += 4;
            }
            2 => {
                // Multiply
                let param_1 = self.param(1, &code)?;
                let param_2 = self.param(2, &code)?;

                let value = param_1 * param_2;
                self.set_param(3, &code, value)?;

                self.counter += 4;
            }
            3 => {
                // Input
                let mut value: Option<i32> = None;

                while value.is_none() {
                    let mut line = String::new();
                    print!("Input: ");
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut line)?;

                    value = line.trim().parse::<i32>().ok();
                }

                self.set_param(
                    1,
                    &code,
                    value.expect("This function is only reched, if `value` is not none!"),
                )?;

                self.counter += 2;
            }
            4 => {
                // Output
                let param_1 = self.param(1, &code)?;

                println!("Output: {}", param_1);

                self.counter += 2;
            }
            5 => {
                // jump if true
                let param_1 = self.param(1, &code)?;
                let param_2 = self.param(2, &code)?;

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

                self.counter += 4;
            }
            8 => {
                // equals than
                let param_1 = self.param(1, &code)?;
                let param_2 = self.param(2, &code)?;

                let value = (param_1 == param_2) as i32;
                self.set_param(3, &code, value)?;

                self.counter += 4;
            }
            99 => {
                self.finished = true;
            }
            _ => {
                return Err(MachineError {
                    message: format!(
                        "Illegal OpCode! Found OpCode '{}' at index {}.",
                        code.op(),
                        self.counter
                    ),
                });
            }
        };

        Ok(())
    }
}
