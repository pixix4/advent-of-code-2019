fn count_digits(layer: &[Vec<u8>], digit: u8) -> usize {
    layer
        .iter()
        .map(|row| row.iter().filter(|d| **d == digit).count())
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input")?;

    let width = 25;
    let height = 6;

    let mut layers: Vec<Vec<Vec<u8>>> = Vec::new();
    let mut rows: Vec<Vec<u8>> = Vec::new();
    let mut cols: Vec<u8> = Vec::new();

    for c in input.chars() {
        match c.to_digit(10) {
            Some(v) => cols.push(v as u8),
            None => continue,
        };

        if cols.len() >= width {
            rows.push(cols);
            cols = Vec::new();
        }

        if rows.len() >= height {
            layers.push(rows);
            rows = Vec::new();
        }
    }

    let min_layer = layers.iter().min_by_key(|l| count_digits(l, 0)).unwrap();

    let one_count = count_digits(min_layer, 1);
    let two_count = count_digits(min_layer, 2);

    println!("prod: {}", one_count * two_count);

    let mut result: Vec<Vec<u8>> = Vec::new();
    for _ in 0..height {
        let mut v: Vec<u8> = Vec::new();
        for _ in 0..width {
            v.push(0);
        }
        result.push(v);
    }

    for layer in layers.iter().rev() {
        for row in 0..height {
            for col in 0..width {
                let curr = layer[row][col];
                if curr != 2 {
                    result[row][col] = curr;
                }
            }
        }
    }

    for row in result {
        for col in row {
            let c = match col {
                0 => ' ',
                1 => '#',
                _ => 'X',
            };
            print!("{}", c);
        }
        println!();
    }

    Ok(())
}
