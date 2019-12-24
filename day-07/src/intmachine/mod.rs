#![allow(dead_code)]

mod error;
mod executer;
mod machine;
mod utils;

pub use error::*;
pub use executer::*;
pub use machine::*;
pub use utils::*;

pub fn parse_file(path: &str) -> MachineResult<Vec<i32>> {
    Ok(parse(&std::fs::read_to_string(path)?))
}

pub fn parse(data: &str) -> Vec<i32> {
    data.split(',')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect()
}
