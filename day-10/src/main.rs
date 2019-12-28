use std::collections::HashMap;

type DegreeVec = Vec<(i32, Vec<(i32, (i32, i32))>)>;

fn get_degree(source: (i32, i32), target: (i32, i32)) -> f32 {
    let vector = (target.0 - source.0, target.1 - source.1);

    let phi = (vector.1 as f32) / ((vector.0.pow(2) + vector.1.pow(2)) as f32).sqrt();
    let angle = phi.acos().to_degrees();

    (if vector.0 < 0 { angle } else { 360.0 - angle } + 180.0) % 360.0
}

fn get_distance(source: (i32, i32), target: (i32, i32)) -> f32 {
    let vector = (target.0 - source.0, target.1 - source.1);
    ((vector.0.pow(2) + vector.1.pow(2)) as f32).sqrt()
}

fn get_coordinates(data: &[Vec<char>]) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = Vec::new();

    for (y, row) in data.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == '#' {
                result.push((x as i32, y as i32));
            }
        }
    }

    result
}

fn build_distance_map(center: (i32, i32), data: &[Vec<char>]) -> DegreeVec {
    let mut degree_map: HashMap<i32, HashMap<i32, (i32, i32)>> = HashMap::new();

    for coord in get_coordinates(data) {
        if center == coord {
            continue;
        }

        let angle = (get_degree(center, coord) * 100.0) as i32;
        let distance = (get_distance(center, coord) * 100.0) as i32;

        degree_map.entry(angle).or_insert_with(HashMap::new);

        degree_map.get_mut(&angle).unwrap().insert(distance, coord);
    }

    let mut degree_vec: DegreeVec = degree_map
        .into_iter()
        .map(|(degree, points)| {
            let mut vec: Vec<(i32, (i32, i32))> = points.into_iter().collect();

            vec.sort_by_key(|(k, _)| *k);

            (degree, vec)
        })
        .collect();
    degree_vec.sort_by_key(|(k, _)| *k);

    degree_vec
}

fn main() {
    let input: Vec<Vec<char>> = std::fs::read_to_string("input")
        .unwrap()
        .split('\n')
        .map(|x| x.chars().collect::<Vec<char>>())
        .filter(|x| !x.is_empty())
        .collect();

    let best = get_coordinates(&input)
        .into_iter()
        .map(|coord| (coord, build_distance_map(coord, &input).len()))
        .max_by_key(|(_, c)| *c)
        .unwrap();

    println!("{:?}", best);

    let mut vec = build_distance_map(best.0, &input);

    let mut index = 0;
    let mut number = 0;
    loop {
        index %= vec.len();
        number += 1;

        let removed_item = vec.get_mut(index).unwrap().1.remove(0);

        if number == 200 {
            let (x, y) = removed_item.1;
            println!(
                "{}: {:?} deg={}° dist={}",
                number, removed_item.1, vec[index].0, removed_item.0
            );
            println!("{}", x * 100 + y);
            break;
        }

        if vec[index].1.is_empty() {
            vec.remove(index);
        } else {
            index += 1;
        }

        if vec.is_empty() {
            println!("All destroyed");
            break;
        }
    }
}
