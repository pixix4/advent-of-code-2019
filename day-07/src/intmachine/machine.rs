use std::sync::mpsc::{channel, Receiver, Sender};

use super::{AllocationMode, Executer};

pub type Interface = (Sender<i32>, Receiver<i32>);

#[derive(Debug)]
pub struct Machine {
    pub program: Vec<i32>,
    pub allocation_mode: AllocationMode,
    executer_count: i32,
}

impl Machine {
    pub fn new(program: &[i32]) -> Self {
        Machine {
            program: program.to_owned(),
            allocation_mode: AllocationMode::AtWrite,
            executer_count: 0,
        }
    }

    pub fn spawn(&mut self) -> Interface {
        let (in_sender, in_receiver) = channel::<i32>();
        let (out_sender, out_receiver) = channel::<i32>();

        let mut executer = Executer::new(
            self.executer_count,
            &self.program,
            in_receiver,
            out_sender,
            AllocationMode::AtWrite,
            false,
        );

        self.executer_count += 1;

        std::thread::spawn(move || {
            executer.run();
        });

        (in_sender, out_receiver)
    }
}
