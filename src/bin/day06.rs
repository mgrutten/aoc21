use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn part1(ages: &HashMap<u8, u64>, days: u32) {
    let mut new_ages = ages.clone();
    for _ in 0..days {
        let mut temp_ages = HashMap::new();
        for (age, count) in new_ages.iter() {
            if *age > 0 {
                *temp_ages.entry(*age - 1).or_insert(0) += count;
            } else {
                *temp_ages.entry(6).or_insert(0) += count;
                *temp_ages.entry(8).or_insert(0) += count;
            }
        }
        new_ages = temp_ages;
    }

    println!("fish count: {}", new_ages.values().sum::<u64>());
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day06/day06.txt")?;

    let numbers = file_str.split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    // Populate a histogram
    let mut ages = HashMap::new();
    for number in numbers {
        *ages.entry(number).or_insert(0_u64) += 1;
    }

    part1(&ages, 80);
    part1(&ages, 256);

    Ok(())
}