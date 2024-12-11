use std::error::Error;
use std::fs;

fn decimal_values(binary: &Vec<u8>) -> (u64, u64) {
    let mut val = 0;
    let mut inverted = 0;
    for (i, &bit) in binary.iter().rev().enumerate() {
        if bit == 1 {
            val += 2_u64.pow(i as u32);
        } else {
            inverted += 2_u64.pow(i as u32);
        }
    }
    (val, inverted)
}


fn part1(numbers: &Vec<Vec<u8>>) {
    let number_count = numbers.len();
    let digit_count = numbers[0].len();

    let gamma_rate_binary = numbers.iter()
        .fold(vec![0_usize; digit_count],
              |acc, v| acc.iter().zip(v.iter())
                  .map(|(a, b)| a + *b as usize)
                  .collect()).iter()
        .map(|v| if *v > number_count / 2 { 1 } else { 0 })
        .collect::<Vec<u8>>();

    let (gamma_rate, epsilon_rate) = decimal_values(&gamma_rate_binary);

    println!("Part 1: {}", gamma_rate * epsilon_rate);
}

fn part2(numbers: &Vec<Vec<u8>>) {
    let digit_count = numbers[0].len();

    let mut o2_numbers = numbers.clone();
    for col in 0..digit_count {
        let number_count = o2_numbers.len();

        let bit_count = o2_numbers.iter()
            .map(|n| n[col] as usize)
            .sum::<usize>();

        let bit_keep = if bit_count * 2 >= number_count { 1 } else { 0 };
        o2_numbers = o2_numbers.into_iter()
            .filter(|n| n[col] == bit_keep)
            .collect::<Vec<Vec<u8>>>();

        if o2_numbers.len() == 1 {
            break;
        }
    }

    let mut co2_numbers = numbers.clone();
    for col in 0..digit_count {
        let number_count = co2_numbers.len();

        let bit_count = co2_numbers.iter()
            .map(|n| n[col] as usize)
            .sum::<usize>();

        let bit_keep = if bit_count * 2 >= number_count { 0 } else { 1 };
        co2_numbers = co2_numbers.into_iter()
            .filter(|n| n[col] == bit_keep)
            .collect::<Vec<Vec<u8>>>();

        if co2_numbers.len() == 1 {
            break;
        }
    }

    let o2_rating = decimal_values(&o2_numbers[0]).0;
    let co2_rating = decimal_values(&co2_numbers[0]).0;

    println!("Part 2: {}", o2_rating * co2_rating);
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day03/day03.txt")?;

    let numbers = file_str.lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect::<Vec<Vec<u8>>>();

    part1(&numbers);
    part2(&numbers);

    Ok(())
}