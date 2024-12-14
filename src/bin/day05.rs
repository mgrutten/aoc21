use std::cmp::{max, Ordering};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Coordinate {
    from: [i32; 2],
    to: [i32; 2],
}

fn get_coord(data: &str) -> [i32; 2] {
    data.split(',')
        .map(|c| c.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn step(start: i32, end: i32) -> i32 {
    match end.cmp(&start) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

fn add_coords(filled: &mut HashMap<[i32; 2], u32>, start: [i32; 2], end: [i32; 2]) {
    let row_step = step(start[0], end[0]);
    let col_step = step(start[1], end[1]);
    let steps = max(
        (end[0] - start[0]).abs() + 1,
        (end[1] - start[1]).abs() + 1,
    );

    let mut row = start[0];
    let mut col = start[1];
    for _ in 0..steps {
        *filled.entry([col, row]).or_insert(0) += 1;
        row += row_step;
        col += col_step;
    }
}


fn part1(coordinates: &Vec<Coordinate>) {
    let mut filled = HashMap::new();
    for coord in coordinates.iter() {
        if coord.from[0] == coord.to[0] || coord.from[1] == coord.to[1] {
            add_coords(&mut filled, coord.from, coord.to);
        }
    }

    let overlap = filled.values().into_iter()
        .filter(|&v| *v > 1)
        .count();
    println!("Part 1: {}", overlap);
}


fn part2(coordinates: &Vec<Coordinate>) {
    let mut filled = HashMap::new();
    for coord in coordinates.iter() {
        add_coords(&mut filled, coord.from, coord.to);
    }

    let overlap = filled.values().into_iter()
        .filter(|&v| *v > 1)
        .count();
    println!("Part 2: {}", overlap);
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day05/day05.txt")?;

    let mut coordinates = Vec::new();
    for line in file_str.lines() {
        let sub_str = line.split_ascii_whitespace().collect::<Vec<&str>>();
        coordinates.push(Coordinate {
            from: get_coord(sub_str[0]),
            to: get_coord(sub_str[2]),
        })
    }

    part1(&coordinates);
    part2(&coordinates);

    Ok(())
}