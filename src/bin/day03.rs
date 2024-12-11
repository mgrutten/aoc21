use std::error::Error;
use std::fs;
use array2d::Array2D;

fn part1(digits: &Array2D<u8>) {

    let digit_count = digits.num_rows();

    let gamma_rate_binary = digits.columns_iter()
        .map(|c| c.fold(0_usize, |acc, v| acc + *v as usize))
        .map(|v| if v > digit_count / 2 {1} else {0})
        .collect::<Vec<u8>>();

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for (i, &bit) in gamma_rate_binary.iter().rev().enumerate() {
        if bit == 1 {
            gamma_rate += 2_u64.pow(i as u32);
        } else {
            epsilon_rate += 2_u64.pow(i as u32);
        }
    }

    println!("Part 1: {}", gamma_rate * epsilon_rate);
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day03/day03.txt")?;

    let map_vec = file_str.lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect::<Vec<Vec<u8>>>();

    let digits = Array2D::from_rows(&map_vec).unwrap();

    part1(&digits);

    Ok(())
}