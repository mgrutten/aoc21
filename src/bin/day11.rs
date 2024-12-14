use array2d::Array2D;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

/*
fn print_levels(energy_levels: &Array2D<u8>) {
    // Increment all
    for row in 0..energy_levels.num_rows() {
        for col in 0..energy_levels.num_columns() {
            print!("{}", energy_levels[(row, col)]);
        }
        println!();
    }
}
 */


fn increment(energy_levels: &mut Array2D<u8>) -> HashSet<(usize, usize)> {
    let mut flashes = HashSet::new();
    for row in 0..energy_levels.num_rows() {
        for col in 0..energy_levels.num_columns() {
            energy_levels[(row, col)] = (energy_levels[(row, col)] + 1) % 10;
            if energy_levels[(row, col)] == 0 {
                flashes.insert((row, col));
            }
        }
    }

    flashes
}

fn flash(energy_levels: &mut Array2D<u8>, flashes: &mut HashSet<(usize, usize)>) {
    let mut new_flashes = HashSet::new();
    for flash in flashes.iter() {
        for row_incr in -1..2 {
            for col_incr in -1..2 {
                let new_row = flash.0 as i32 + row_incr;
                let new_col = flash.1 as i32 + col_incr;
                if new_row >= 0 && new_row < energy_levels.num_rows() as i32 &&
                    new_col >= 0 && new_col < energy_levels.num_columns() as i32 &&
                    energy_levels[(new_row as usize, new_col as usize)] != 0 {
                    // New location
                    let new_location = (new_row as usize, new_col as usize);
                    energy_levels[new_location] = (energy_levels[new_location] + 1) % 10;
                    if energy_levels[new_location] == 0 {
                        new_flashes.insert(new_location);
                    }
                }
            }
        }
    }

    if new_flashes.len() > 0 {
        flash(energy_levels, &mut new_flashes);
        flashes.extend(new_flashes);
    }
}


fn part1(energy_levels: &Array2D<u8>) {
    let mut new_levels = energy_levels.clone();

    let mut total_flashes = 0;
    for _ in 0..100 {
        // Increment all
        let mut flashes = increment(&mut new_levels);

        // Increment flash neighbours
        flash(&mut new_levels, &mut flashes);
        total_flashes += flashes.len();
    }

    println!("Part 1: {}", total_flashes);
}


fn part2(energy_levels: &Array2D<u8>) {
    let mut new_levels = energy_levels.clone();

    let mut steps = 1;
    loop {
        // Increment all
        let mut flashes = increment(&mut new_levels);

        // Increment flash neighbours
        flash(&mut new_levels, &mut flashes);

        let flash_count = new_levels.elements_row_major_iter()
            .map(|e| if *e == 0 { 1 } else { 0 })
            .sum::<usize>();

        if flash_count == energy_levels.num_elements() {
            break;
        }
        steps += 1;
    }

    println!("Part 2: {}", steps);
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day11/day11.txt")?;

    let mut rows = Vec::new();
    for line in file_str.lines() {
        rows.push(line.bytes().map(|b| b - b'0').collect::<Vec<_>>());
    }

    let energy_levels = Array2D::from_rows(&rows).unwrap();

    part1(&energy_levels);
    part2(&energy_levels);

    Ok(())
}