mod intmachine;

use intmachine::{Interface, Machine, MachineResult};

fn permute(vec: Vec<i32>, place: usize) -> Vec<Vec<i32>> {
    if place >= vec.len() {
        vec![vec]
    } else {
        let mut result: Vec<Vec<i32>> = Vec::new();

        for index in 0..vec.len() {
            let index = index as i32;

            let mut cont = false;
            for item in vec.iter().take(place) {
                if index == *item {
                    cont = true;
                    break;
                }
            }
            if cont {
                continue;
            }

            let mut vec_new = vec.clone();
            vec_new[place] = index;

            result.append(&mut permute(vec_new, place + 1));
        }

        result
    }
}

fn main() -> MachineResult<()> {
    let program = intmachine::parse_file("input")?;

    let mut max_signal: i32 = 0;
    let mut max_phases: Vec<i32> = vec![];

    let phases_permutation = permute(vec![0, 1, 2, 3, 4], 0);

    let mut machine = Machine::new(&program);

    for phases in phases_permutation {
        let phases: Vec<i32> = phases.iter().map(|v| v + 5).collect();
        let mut param = 0;
        let mut last_iteration = -1;

        let mut machines: Vec<Interface> = phases.iter().map(|_| machine.spawn()).collect();

        for (index, (sender, _)) in machines.iter_mut().enumerate() {
            sender.send(phases[index])?;
        }

        while last_iteration != param {
            last_iteration = param;
            for (sender, receiver) in machines.iter_mut() {
                if sender.send(param).is_err() {
                    break;
                }

                param = receiver.recv().unwrap_or(last_iteration);
            }
        }

        if param > max_signal {
            max_signal = param;
            max_phases = phases;
        }
    }

    println!("Signal: {}, Phases: {:?}", max_signal, max_phases);

    Ok(())
}
