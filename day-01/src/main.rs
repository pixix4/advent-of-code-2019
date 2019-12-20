use std::fs;

fn calc_fuel(module: i32) -> i32 {
    let mut sum = 0;
    let mut last_fuel = module;

    loop {
        last_fuel = last_fuel / 3 - 2;

        if last_fuel <= 0 {
            break;
        }

        sum += last_fuel;
    }

    sum
}

fn main() -> std::io::Result<()> {
    let sum: i32 = fs::read_to_string("input")?
        .split("\n")
        .filter_map(|s| s.parse::<i32>().ok())
        .map(calc_fuel)
        .sum();

    println!("Sum is: {}", sum);

    Ok(())
}
