#![allow(dead_code)]

mod error;
mod executer;
mod interface;
mod machine;
mod utils;

pub use error::*;
pub use executer::*;
pub use interface::*;
pub use machine::*;
pub use utils::*;

pub fn parse_file(path: &str) -> MachineResult<Vec<i128>> {
    Ok(parse(&std::fs::read_to_string(path)?))
}

pub fn parse(data: &str) -> Vec<i128> {
    data.split(',')
        .filter_map(|s| s.trim().parse::<i128>().ok())
        .collect()
}
