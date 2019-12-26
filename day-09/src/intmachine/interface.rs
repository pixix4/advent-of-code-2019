use std::sync::mpsc::{channel, Receiver, Sender};

use super::MachineResult;

pub trait MachineInterface {
    fn send(&mut self, value: i128) -> MachineResult<()>;
    fn receive(&mut self) -> MachineResult<i128>;
}

pub struct ChannelInterface {
    pub in_sender: Sender<i128>,
    in_receiver: Receiver<i128>,
    out_sender: Sender<i128>,
    pub out_receiver: Receiver<i128>,
}

impl ChannelInterface {
    pub fn new() -> Self {
        let (in_sender, in_receiver) = channel::<i128>();
        let (out_sender, out_receiver) = channel::<i128>();

        ChannelInterface {
            in_sender,
            in_receiver,
            out_sender,
            out_receiver,
        }
    }
}
impl MachineInterface for ChannelInterface {
    fn send(&mut self, value: i128) -> MachineResult<()> {
        self.out_sender.send(value)?;
        Ok(())
    }

    fn receive(&mut self) -> MachineResult<i128> {
        let value = self.in_receiver.recv()?;
        Ok(value)
    }
}

pub struct IOInterface {}
impl IOInterface {
    pub fn new() -> Self {
        IOInterface {}
    }
}
impl MachineInterface for IOInterface {
    fn send(&mut self, value: i128) -> MachineResult<()> {
        println!("Output: {}", value);
        Ok(())
    }

    fn receive(&mut self) -> MachineResult<i128> {
        print!("Input: ");
        use std::io::Write;
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        Ok(input.trim().parse().unwrap())
    }
}
unsafe impl std::marker::Send for IOInterface {}
