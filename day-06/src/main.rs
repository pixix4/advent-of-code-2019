use std::collections::HashMap;

fn path_length(map: &HashMap<String, String>, value: &str) -> i32 {
    build_path(map, value).len() as i32
}

fn build_path(map: &HashMap<String, String>, value: &str) -> Vec<String> {
    if !map.contains_key(value) {
        vec![]
    } else {
        let mut path = build_path(map, &map[value]);
        path.push(value.to_owned());
        path
    }
}

fn main() -> std::io::Result<()> {
    let map: HashMap<String, String> = std::fs::read_to_string("input")?
        .split('\n')
        .filter_map(|s| {
            let line: Vec<&str> = s.split(')').collect();
            if line.len() == 2 {
                Some((line[1].to_owned(), line[0].to_owned()))
            } else {
                None
            }
        })
        .collect();

    let checksum: i32 = map.keys().map(|key| path_length(&map, key)).sum();

    println!("Checksum: {}", checksum);

    let mut you_path = build_path(&map, "YOU");
    let mut san_path = build_path(&map, "SAN");

    while !you_path.is_empty() && !san_path.is_empty() && you_path[0] == san_path[0] {
        you_path.remove(0);
        san_path.remove(0);
    }

    println!("Length: {}", you_path.len() + san_path.len() - 2);

    Ok(())
}
