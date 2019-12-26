use super::Executer;

#[derive(Debug)]
pub struct Machine {
    pub program: Vec<i128>,
    executer_count: i128,
}

impl Machine {
    pub fn new(program: &[i128]) -> Self {
        Machine {
            program: program.to_owned(),
            executer_count: 0,
        }
    }

    pub fn spawn(&mut self) {
        let interface = super::IOInterface::new();

        let number = self.executer_count;
        let program = self.program.clone();
        self.executer_count += 1;

        // std::thread::spawn(move || {
        let mut executer = Executer::new(number, &program, Box::new(interface), false);

        executer.run();
        // });
    }
}
