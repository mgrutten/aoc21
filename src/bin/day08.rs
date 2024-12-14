use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Problem {
    digits: [HashSet<char>; 10],
    output: [HashSet<char>; 4],
}

fn part1(problems: &Vec<Problem>) {
    let mut digit_count = 0;
    for problem in problems {
        for segments in problem.output.iter() {
            if segments.len() == 2 || segments.len() == 3 ||
                segments.len() == 4 || segments.len() == 7 {
                digit_count += 1;
            }
        }
    }

    println!("Part 1: {}", digit_count);
}


fn part2(problems: &Vec<Problem>) {
    // Form a hash for each digit
    // Count up the number of times each segment appears in 0-9 (first set of pairs)
    // Sum up the segment counts for each digit, which is unique
    let pairs = vec![
        ('a', 8),
        ('b', 6),
        ('c', 8),
        ('d', 7),
        ('e', 4),
        ('f', 9),
        ('g', 7),
    ];
    let segment_counts = pairs.into_iter().collect::<HashMap<_, _>>();

    let pairs = vec![
        (0_u64, "abcefg"),
        (1, "cf"),
        (2, "acdeg"),
        (3, "acdfg"),
        (4, "bcdf"),
        (5, "abdfg"),
        (6, "abdefg"),
        (7, "acf"),
        (8, "abcdefg"),
        (9, "abcdfg"),
    ];

    // This results in (sum of segment counts: digit)
    // {45: 9, 42: 0, 39: 3, 49: 8, 30: 4, 37: 5, 25: 7, 17: 1, 41: 6, 34: 2}
    let value_hash = pairs.into_iter()
        .map(|(digit, segments)| (segments.chars()
                                      .map(|c| segment_counts[&c])
                                      .sum::<u32>(), digit)
        )
        .collect::<HashMap<_, _>>();

    let mut value_sum = 0;
    for problem in problems {
        let mut segment_counts = HashMap::new();
        for segment in ['a', 'b', 'c', 'd', 'e', 'f', 'g'].iter() {
            for digit in &problem.digits {
                if digit.contains(segment) {
                    *segment_counts.entry(segment).or_insert(0) += 1;
                }
            }
        }
        let output_digits = problem.output.iter()
            .map(|d| d.iter().map(|s| segment_counts[s]).sum::<u32>())
            .map(|v| value_hash[&v])
            .collect::<Vec<_>>();

        let value = 1000 * output_digits[0] + 100 * output_digits[1] + 10 * output_digits[2] + output_digits[3];
        value_sum += value;
    }

    println!("Part 2: {:?}", value_sum);
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day08/day08.txt")?;

    let mut problems = Vec::new();
    for line in file_str.lines() {
        let lr = line.split('|').collect::<Vec<&str>>();

        let digits: [HashSet<char>; 10] = lr[0].split_ascii_whitespace()
            .map(|s| s.chars().collect::<HashSet<char>>())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let output: [HashSet<char>; 4] = lr[1].split_ascii_whitespace()
            .map(|s| s.chars().collect::<HashSet<char>>())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        problems.push(Problem { digits, output });
    }
    
    part1(&problems);
    part2(&problems);

    Ok(())
}