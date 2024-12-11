use std::error::Error;
use std::fs;

fn part1(directions: &Vec<(&str, i32)>) {
    let mut location = (0, 0);
    for (direction, distance) in directions {
        match *direction {
            "forward" => location.1 += distance,
            "down" => location.0 += distance,
            "up" => location.0 -= distance,
            _ => {
                panic!("Unknown direction: {}", direction);
            }
        }
    }

    println!("Part 1: {}", location.0 * location.1);
}


fn part2(directions: &Vec<(&str, i32)>) {
    let mut location = (0, 0, 0);
    for (direction, distance) in directions {
        match *direction {
            "forward" => {
                location.1 += distance;
                location.0 += location.2 * distance;
            },
            "down" => location.2 += distance,
            "up" => location.2 -= distance,
            _ => {
                panic!("Unknown direction: {}", direction);
            }
        }
    }
    
    println!("Part 2: {}", location.0 * location.1);
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day02/day02.txt")?;

    let directions = file_str.lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|v| (v[0], v[1].parse::<i32>().unwrap()))
        .collect::<Vec<(&str, i32)>>();

    part1(&directions);
    part2(&directions);

    Ok(())
}