use std::cmp::{max, Ordering};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Coordinate {
    from: [u32; 2],
    to: [u32; 2],
}

fn get_coord(data: &str) -> [u32; 2] {
    data.split(',')
        .map(|c| c.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}


fn print_map(filled: &HashMap<[u32; 2], u32>) {
    let rows = filled.keys()
        .map(|&c| c[0])
        .max()
        .unwrap() + 1;

    let cols = filled.keys()
        .map(|&c| c[1])
        .max()
        .unwrap() + 1;

    //println!("{} {} {}", rows, cols, filled.len());

    for row in 0..rows {
        for col in 0..cols {
            if filled.contains_key(&[row, col]) {
                print!("{}", filled[&[row, col]]);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn add_row(filled: &mut HashMap<[u32; 2], u32>, row: u32, col_start: u32, col_end: u32) {
    if col_start <= col_end {
        for col in col_start..col_end + 1 {
            *filled.entry([row, col]).or_insert(0) += 1;
        }
    } else {
        for col in col_end..col_start + 1 {
            *filled.entry([row, col]).or_insert(0) += 1;
        }
    }
}


fn add_col(filled: &mut HashMap<[u32; 2], u32>, col: u32, row_start: u32, row_end: u32) {
    if row_start <= row_end {
        for row in row_start..row_end + 1 {
            *filled.entry([row, col]).or_insert(0) += 1;
        }
    } else {
        for row in row_end..row_start + 1 {
            *filled.entry([row, col]).or_insert(0) += 1;
        }
    }
}

fn step(start: i32, end: i32) -> i32 {
    match end.cmp(&start) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

fn add_diag(filled: &mut HashMap<[u32; 2], u32>, start: [u32; 2], end: [u32; 2]) {
    let row_step = step(start[0] as i32, end[0] as i32);
    let col_step = step(start[1] as i32, end[1] as i32);
    let steps = max(
        (end[0] as i32 - start[0] as i32).abs() + 1,
        (end[1] as i32 - start[1] as i32).abs() + 1,
    );

    let mut row = start[0] as i32;
    let mut col = start[1] as i32;
    for _ in 0..steps {
        *filled.entry([col as u32, row as u32]).or_insert(0) += 1;
        row += row_step;
        col += col_step;
    }
}


fn part1(coordinates: &Vec<Coordinate>) {
    let mut filled = HashMap::new();
    for coord in coordinates.iter() {
        if coord.from[0] == coord.to[0] {
            //println!("{:?}", coord);
            add_col(&mut filled, coord.from[0], coord.from[1], coord.to[1]);
        }

        if coord.from[1] == coord.to[1] {
            //println!("{:?}", coord);
            add_row(&mut filled, coord.from[1], coord.from[0], coord.to[0]);
        }

        //print_map(&filled);
        //println!();
    }

    //print_map(&filled);
    //println!("{:#?}", filled);

    let overlap = filled.values().into_iter()
        .filter(|&v| *v > 1)
        .count();
    println!("Part 1: {}", overlap);
}


fn part2(coordinates: &Vec<Coordinate>) {
    let mut filled = HashMap::new();
    for coord in coordinates.iter() {
        //if coord.from[0] == coord.to[0] || coord.from[1] == coord.to[1] {
        //println!("{:?}", coord);
        add_diag(&mut filled, coord.from, coord.to);
        //}

        //print_map(&filled);
        //println!();
    }

    //print_map(&filled);
    //println!("{:#?}", filled);

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

    //println!("{:?}", coordinates);
    part1(&coordinates);
    part2(&coordinates);

    Ok(())
}