use std::collections::HashMap;
use std::error::Error;
use std::fs;

enum ParseResult {
    OK(usize),
    Error(char),
    Incomplete(Vec<char>),
}

fn process_chunk(line: &Vec<char>, index: usize, closing: &HashMap<char, char>) -> ParseResult {
    if index + 1 >= line.len() {
        return ParseResult::Incomplete(vec![closing[&line[index]]]);
    }

    let mut end_index = index + 1;
    while closing.contains_key(&line[end_index]) {
        match process_chunk(line, end_index, closing) {
            ParseResult::OK(process_index) => end_index = process_index,
            ParseResult::Error(bad_terminator) => return ParseResult::Error(bad_terminator),
            ParseResult::Incomplete(terminators) => {
                let mut new_terminators = terminators.clone();
                new_terminators.push(closing[&line[index]]);
                return ParseResult::Incomplete(new_terminators);
            }
        }

        if end_index >= line.len() {
            return ParseResult::Incomplete(vec![closing[&line[index]]]);
        }
    }

    if line[end_index] == closing[&line[index]] {
        ParseResult::OK(end_index + 1)
    } else {
        ParseResult::Error(line[end_index])
    }
}


fn process_chunks(line: &Vec<char>, closing: &HashMap<char, char>) -> ParseResult {
    let mut end_index = 0;
    while closing.contains_key(&line[end_index]) {
        match process_chunk(line, end_index, closing) {
            ParseResult::OK(process_index) => end_index = process_index,
            ParseResult::Error(bad_terminator) => return ParseResult::Error(bad_terminator),
            ParseResult::Incomplete(terminators) => return ParseResult::Incomplete(terminators),
        }
    }

    ParseResult::OK(0)
}


fn part1(lines: &Vec<Vec<char>>) {
    let closing_bracket = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    let scores = HashMap::from([
        (')', 3_u64),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);

    let mut error_score = 0;
    for line in lines {
        match process_chunks(line, &closing_bracket) {
            ParseResult::OK(_) => {}
            ParseResult::Error(bad_terminator) => error_score += scores[&bad_terminator],
            ParseResult::Incomplete(_) => {}
        }
    }

    println!("Part 1: {}", error_score);
}


fn part2(lines: &Vec<Vec<char>>) {
    let closing_bracket = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    let scores = HashMap::from([
        (')', 1_u64),
        (']', 2),
        ('}', 3),
        ('>', 4),
    ]);

    let mut error_scores = Vec::new();
    for line in lines {
        match process_chunks(line, &closing_bracket) {
            ParseResult::OK(_) => {}
            ParseResult::Error(_) => {}
            ParseResult::Incomplete(terminators) => {
                let error_score = terminators.iter().fold(0, |acc, c| acc * 5 + scores[c]);
                error_scores.push(error_score);
            }
        }
    }

    error_scores.sort();
    println!("Part 2: {}", error_scores[error_scores.len() / 2]);
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day10/day10.txt")?;

    let mut lines = Vec::new();
    for l in file_str.lines() {
        lines.push(l.chars().collect::<Vec<_>>());
    }

    part1(&lines);
    part2(&lines);

    Ok(())
}