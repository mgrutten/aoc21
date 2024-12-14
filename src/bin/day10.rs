use std::collections::HashMap;
use std::error::Error;
use std::fs;

enum ParseResult {
    OK(usize),
    Error(char),
    Incomplete,
}

fn process_chunk(line: &Vec<char>, index: usize, closing: &HashMap<char, char>) -> ParseResult {
    if index >= line.len() - 1 {
        return ParseResult::Incomplete;
    }

    let mut end_index = index + 1;
    while closing.contains_key(&line[end_index]) {
        match process_chunk(line, end_index, closing) {
            ParseResult::OK(process_index) => end_index = process_index,
            ParseResult::Error(bad_terminator) => return ParseResult::Error(bad_terminator),
            ParseResult::Incomplete => return ParseResult::Incomplete,
        }

        if end_index >= line.len() {
            return ParseResult::Incomplete;
        }
    }

    if line[end_index] == closing[&line[index]] {
        ParseResult::OK(end_index + 1)
    } else {
        ParseResult::Error(line[end_index])
    }
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
        match process_chunk(line, 0, &closing_bracket) {
            ParseResult::OK(_) => println!("complete"),
            ParseResult::Error(bad_terminator) => {
                error_score += scores[&bad_terminator];
                println!("problem at {}", bad_terminator);
            },
            ParseResult::Incomplete => println!("incomplete"),
        }
    }
    
    println!("Part 1: {}", error_score);
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day10/day10.txt")?;

    let mut lines = Vec::new();
    for l in file_str.lines() {
        lines.push(l.chars().collect::<Vec<_>>());
    }

    part1(&lines);

    Ok(())
}