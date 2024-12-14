use std::error::Error;
use std::fs;


fn part1(positions: &Vec<i32>) {
    let mut min_fuel = u64::MAX;
    let mut align_pos = 0;
    loop {
        let mut fuel: u64 = 0;
        for pos in positions {
            fuel += (pos - align_pos).abs() as u64
        }

        // Convex problem, so break once we're at the minimum
        if fuel < min_fuel {
            min_fuel = fuel;
        } else {
            break;
        }

        align_pos += 1;
    }

    println!("Part 1: {}", min_fuel);
}


fn part2(positions: &Vec<i32>) {
    let mut min_fuel = u64::MAX;
    let mut align_pos = 0;
    loop {
        let mut fuel: u64 = 0;
        for pos in positions {
            let diff = (pos - align_pos).abs() as u64;
            fuel += diff * (diff + 1) / 2;
        }

        // Convex problem, so break once we're at the minimum
        if fuel < min_fuel {
            min_fuel = fuel;
        } else {
            break;
        }

        align_pos += 1;
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