use std::error::Error;
use std::fs;


fn part1(depths: &Vec<u32>) {
    let increases = depths.windows(2)
        .map(|window| window[1] > window[0])
        .filter(|&difference| difference)
        .count();

    println!("Increases: {}", increases);
}


fn part2(depths: &Vec<u32>) {
    let windows = depths.windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<u32>>();

    part1(&windows);
    //println!("Part 1: {}", increases);
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day01/day01.txt")?;

    let depths = file_str.lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    part1(&depths);
    part2(&depths);

    Ok(())
}