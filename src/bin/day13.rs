use std::collections::HashSet;
use std::error::Error;
use std::fs;


fn do_fold(locations: &HashSet<[usize; 2]>, fold: &(char, usize)) -> HashSet<[usize; 2]> {
    let mut new_locations = HashSet::new();
    for loc in locations.iter() {
        if fold.0 == 'x' {
            if loc[0] > fold.1 {
                new_locations.insert([2 * fold.1 - loc[0], loc[1]]);
            } else {
                new_locations.insert(*loc);
            }
        } else {
            if loc[1] > fold.1 {
                new_locations.insert([loc[0], 2 * fold.1 - loc[1]]);
            } else {
                new_locations.insert(*loc);
            }
        }
    }

    new_locations
}


fn part1(locations: &HashSet<[usize; 2]>, fold: &(char, usize)) {
    let new_locations = do_fold(locations, fold);
    println!("Locations: {:?}", new_locations.len());
}


fn print_dots(locations: &HashSet<[usize; 2]>) {
    let max_x = locations.iter().map(|loc| loc[0]).max().unwrap();
    let max_y = locations.iter().map(|loc| loc[1]).max().unwrap();

    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            if locations.contains(&[x, y]) {
                print!("O");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}


fn part2(locations_in: &HashSet<[usize; 2]>, folds: &Vec<(char, usize)>) {
    let mut locations = locations_in.clone();
    for fold in folds {
        locations = do_fold(&locations, fold);
    }

    print_dots(&locations);
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day13/day13.txt")?;

    let lines = file_str.lines().collect::<Vec<&str>>();
    let mut idx = 0;

    let mut locations = HashSet::new();
    while lines[idx].len() > 0 {
        let lr = lines[idx].split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        locations.insert([lr[0], lr[1]]);
        idx += 1;
    }

    idx += 1;
    let mut folds = Vec::new();
    while idx < lines.len() {
        let lr = lines[idx][11..].split('=').collect::<Vec<&str>>();
        folds.push((lr[0].chars().collect::<Vec<_>>()[0], lr[1].parse::<usize>().unwrap()));

        idx += 1;
    }

    part1(&locations, &folds[0]);
    part2(&locations, &folds);

    Ok(())
}