mod machine;

use machine::Machine;

fn parse(data: &str) -> Vec<i32> {
    data.split(',')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

fn main() -> std::io::Result<()> {
    let program = parse(&std::fs::read_to_string("input")?);

    let mut machine = Machine::new(&program);
    let result = machine.run();

    println!("Successful: {:?}", result);

    Ok(())
}
