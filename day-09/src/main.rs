mod intmachine;

use intmachine::{Machine, MachineResult};

fn main() -> MachineResult<()> {
    //let program = intmachine::parse("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    let program = intmachine::parse_file("input")?;

    let mut machine = Machine::new(&program);

    machine.spawn();

    Ok(())
}
