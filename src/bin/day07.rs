use std::cmp::min;
use std::error::Error;
use std::fs;


fn part1(positions: &Vec<i32>) {
    let max_position = positions.iter().max().unwrap() + 1;

    let mut min_fuel = u64::MAX;
    for align_pos in 0..max_position {
        let mut fuel: u64 = 0;
        for pos in positions {
            fuel += (pos - align_pos).abs() as u64
        }
        min_fuel = min(min_fuel, fuel);
    }

    println!("Part 1: {}", min_fuel);
}


fn part2(positions: &Vec<i32>) {
    let max_position = positions.iter().max().unwrap() + 1;

    let mut min_fuel = u64::MAX;
    for align_pos in 0..max_position {
        let mut fuel: u64 = 0;
        for pos in positions {
            let diff = (pos - align_pos).abs() as u64;
            fuel += diff * (diff + 1) / 2;
        }
        min_fuel = min(min_fuel, fuel);
    }

    println!("Part 2: {}", min_fuel);
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day07/day07.txt")?;

    let positions = file_str.split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    part1(&positions);
    part2(&positions);

    Ok(())
}