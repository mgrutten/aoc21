use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn pair_insertions(pairs: &HashMap<[char; 2], u64>,
                   rules: &Vec<([char; 2], char)>,
                   last_symbol: char,
                   iterations: usize, ) {
    let mut mut_pairs = pairs.clone();

    // Do the insertions
    for _ in 0..iterations {
        let mut new_pairs = HashMap::new();
        for (pair, split) in rules {
            if let Some(pair_count) = mut_pairs.get(pair) {
                *new_pairs.entry([pair[0], *split]).or_insert(0) += pair_count;
                *new_pairs.entry([*split, pair[1]]).or_insert(0) += pair_count;
            }
        }
        mut_pairs = new_pairs;
    }

    // Get symbol counts, being careful with the last symbol
    let mut symbols = HashMap::new();
    mut_pairs.iter()
        .for_each(|(pair, count)| {
            *symbols.entry(pair[0]).or_insert(0) += count;
        });
    *symbols.entry(last_symbol).or_insert(0) += 1;

    let max_count = symbols.values().max().unwrap();
    let min_count = symbols.values().min().unwrap();

    println!("Part 1: {}", max_count - min_count);
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day14/day14.txt")?;

    let mut lines = file_str.lines();
    let line0 = lines.next().unwrap();

    let last_char = line0.chars().last().unwrap();
    let pair_vec = line0.chars().collect::<Vec<char>>()
        .windows(2)
        .map(|pair| [pair[0], pair[1]])
        .collect::<Vec<_>>();
    let mut pairs = HashMap::new();
    pair_vec.iter()
        .for_each(|pair| *pairs.entry(*pair).or_insert(0) += 1);

    lines.next();
    let mut rules = Vec::new();
    for line in lines {
        let chars = line.chars().collect::<Vec<char>>();
        rules.push(([chars[0], chars[1]], chars[6]));
    }

    pair_insertions(&pairs, &rules, last_char, 10);
    pair_insertions(&pairs, &rules, last_char, 40);

    Ok(())
}