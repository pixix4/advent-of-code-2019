use std::fs;

struct Machine {
    pub program: Vec<i32>,
    pub counter: usize,
    pub finished: bool,
}

impl Machine {
    pub fn new(program: &Vec<i32>) -> Self {
        Machine {
            program: program.to_owned(),
            counter: 0,
            finished: false,
        }
    }

    pub fn run(&mut self, params: &Vec<i32>) -> i32 {
        for (index, param) in params.iter().enumerate() {
            self.program[index + 1] = *param;
        }

        while !self.finished {
            let code = self.program[self.counter];

            match code {
                1 => {
                    let index_1 = self.program[self.counter + 1] as usize;
                    let index_2 = self.program[self.counter + 2] as usize;
                    let index_3 = self.program[self.counter + 3] as usize;

                    self.program[index_3] = self.program[index_1] + self.program[index_2];

                    self.counter += 4;
                },
                2 => {
                    let index_1 = self.program[self.counter + 1] as usize;
                    let index_2 = self.program[self.counter + 2] as usize;
                    let index_3 = self.program[self.counter + 3] as usize;

                    self.program[index_3] = self.program[index_1] * self.program[index_2];

                    self.counter += 4;
                },
                99 => {
                    self.finished = true;
                },
                _ => {
                    panic!("Unknown optcode: {} at index: {}", code, self.counter);
                },
            }
        }

        self.program[0]
    }
}


fn main() -> std::io::Result<()> {
    let program: Vec<i32> = fs::read_to_string("input")?.split(",")
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();


    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut machine = Machine::new(&program);
            let result = machine.run(&vec![noun, verb]);

            if result == 19690720 {
                println!("Result is: {}", 100 * noun + verb);
                break;
            }
        }
    }

    Ok(())
}
