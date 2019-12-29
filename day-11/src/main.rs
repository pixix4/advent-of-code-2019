mod intmachine;

use intmachine::{Machine, MachineInterface, MachineResult};

use std::collections::HashSet;

#[derive(Debug)]
struct Map {
    position: (i32, i32),
    direction: i32,
    color_map: HashSet<(i32, i32)>,
    painted_tiles: HashSet<(i32, i32)>,
    toggle_state: bool,
    debug: bool,
}

impl Map {
    pub fn new() -> Self {
        let mut map = Map {
            position: (0, 0),
            direction: 0,
            color_map: HashSet::new(),
            painted_tiles: HashSet::new(),
            toggle_state: false,
            debug: true,
        };

        map.color_map.insert(map.position);

        map
    }

    pub fn print(&self) {
        if self.debug {
            let (mut min_x, mut min_y) = self.position;
            let (mut max_x, mut max_y) = self.position;

            for p in &self.color_map {
                let (x, y) = *p;

                min_x = std::cmp::min(min_x, x);
                min_y = std::cmp::min(min_y, y);
                max_x = std::cmp::max(max_x, x);
                max_y = std::cmp::max(max_y, y);
            }

            min_x -= 2;
            max_x += 2;
            min_y -= 2;
            max_y += 2;

            for row in (min_y..=max_y).rev() {
                for col in min_x..=max_x {
                    let position = (col, row);
                    if position == self.position {
                        print!(
                            "{}",
                            match self.direction {
                                0 => '^',
                                1 => '>',
                                2 => 'v',
                                3 => '<',
                                _ => ' ',
                            }
                        );
                    } else {
                        print!(
                            "{}",
                            if self.color_map.contains(&position) {
                                '#'
                            } else {
                                '.'
                            }
                        );
                    }
                }
                println!();
            }
        }

        println!("Tiles: {}", self.painted_tiles.len());
        println!();

        // std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

impl MachineInterface for Map {
    fn send(&mut self, value: i128) -> MachineResult<()> {
        if !self.toggle_state {
            if value == 1 {
                self.color_map.insert(self.position);
            } else {
                self.color_map.remove(&self.position);
            }
            self.painted_tiles.insert(self.position);
        } else {
            self.direction = match value {
                0 => self.direction + 3,
                1 => self.direction + 1,
                _ => self.direction,
            } % 4;

            let (x, y) = self.position;
            self.position = match self.direction {
                0 => (x, y + 1),
                1 => (x + 1, y),
                2 => (x, y - 1),
                3 => (x - 1, y),
                _ => (x, y),
            };

            self.print();
        }

        self.toggle_state = !self.toggle_state;

        Ok(())
    }

    fn receive(&mut self) -> MachineResult<i128> {
        let value = self.color_map.contains(&self.position) as i128;
        Ok(value)
    }
}

fn main() -> MachineResult<()> {
    //let program = intmachine::parse("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    let program = intmachine::parse_file("input")?;

    let map = Map::new();

    let mut machine = Machine::new(&program);

    machine.spawn(Box::new(map));

    Ok(())
}
