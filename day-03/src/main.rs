use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn step(&self, point: (i32, i32)) -> (i32, i32) {
        let (x, y) = point;
        match self {
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y - 1),
            Direction::Up => (x, y + 1),
        }
    }
}

type Command = (Direction, i32);

fn parse_command(value: &str) -> Option<Command> {
    if value.is_empty() {
        return None;
    }

    let d = match &value[0..1] {
        "D" => Direction::Down,
        "U" => Direction::Up,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => return None,
    };

    let i = match value[1..].parse::<i32>() {
        Ok(i) => i,
        Err(_) => return None,
    };

    Some((d, i))
}

fn main() -> std::io::Result<()> {
    let cables: Vec<Vec<Command>> = fs::read_to_string("input")?
        .split("\n")
        .map(|line| {
            line.split(",")
                .filter_map(parse_command)
                .collect::<Vec<Command>>()
        })
        .filter(|v| !v.is_empty())
        .collect();

    let mut points: HashMap<(i32, i32), i32> = HashMap::new();

    let mut position = (0, 0);
    let mut steps = 0;
    for (direction, count) in &cables[0] {
        for _ in 0..*count {
            position = direction.step(position);
            steps += 1;
            points.insert(position, steps);
        }
    }

    let mut intersections: Vec<((i32, i32), i32)> = Vec::new();

    position = (0, 0);
    let mut steps = 0;
    for (direction, count) in &cables[1] {
        for _ in 0..*count {
            position = direction.step(position);
            steps += 1;
            if points.contains_key(&position) {
                intersections.push((position, steps + points[&position]));
            }
        }
    }

    println!("{:?}", intersections);

    let min = intersections.iter().min_by(|(_, ad), (_, bd)| ad.cmp(bd));

    if let Some(((x, y), d)) = min {
        println!("x: {}, y: {}, dist: {}", x, y, d);
    }

    Ok(())
}
