use std::collections::HashSet;
use array2d::Array2D;
use std::error::Error;
use std::fs;

fn check_winner(board: &Array2D<bool>) -> bool {
    for mut row in board.rows_iter() {
        if row.all(|&c| c) {
            return true;
        }
    }

    for mut col in board.columns_iter() {
        if col.all(|&r| r) {
            return true;
        }
    }

    false
}

fn board_score(board: &Array2D<u64>, tf_board: &Array2D<bool>) -> u64 {
    let mut score = 0;
    for (row, col) in board.indices_row_major() {
        if !tf_board[(row, col)] {
            score += board[(row, col)];
        }
    }

    score
}


fn part1(numbers: &Vec<u64>, boards: &Vec<Array2D<u64>>) {

    // Set up boolean boards
    let mut tf_boards = Vec::new();
    for _ in boards {
        tf_boards.push(Array2D::filled_by_row_major(|| false, 5, 5));
    }

    let mut winning_score = None;
    'outer: for number in numbers {
        for (idx, board) in boards.iter().enumerate() {
            for (row, col) in board.indices_row_major() {
                if board[(row, col)] == *number {
                    // Mark number on board
                    tf_boards[idx][(row, col)] = true;

                    if check_winner(&tf_boards[idx]) {
                        winning_score = Some(number * board_score(&board, &tf_boards[idx]));
                        break 'outer;
                    }
                }
            }
        }
    }

    if let Some(score) = winning_score {
        println!("Part 1: {}", score);
    }
}


fn part2(numbers: &Vec<u64>, boards: &Vec<Array2D<u64>>) {

    // Set up boolean boards
    let mut tf_boards = Vec::new();
    for _ in boards {
        tf_boards.push(Array2D::filled_by_row_major(|| false, 5, 5));
    }

    let mut winning_score = None;
    let mut solved_boards = HashSet::new();
    'outer: for number in numbers {
        for (idx, board) in boards.iter().enumerate() {
            if !solved_boards.contains(&idx) {
                for (row, col) in board.indices_row_major() {
                    if board[(row, col)] == *number {
                        // Mark number on board
                        tf_boards[idx][(row, col)] = true;

                        if check_winner(&tf_boards[idx]) {
                            solved_boards.insert(idx);
                        }

                        if solved_boards.len() == boards.len() {
                            winning_score = Some(number * board_score(&board, &tf_boards[idx]));
                            break 'outer;
                        }
                    }
                }
            }
        }
    }

    if let Some(score) = winning_score {
        println!("Part 2: {}", score);
    }
}



fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day04/day04.txt")?;

    let mut lines = file_str.lines();

    // First line
    let numbers = lines.next().unwrap().split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // Skip second line
    lines.next();

    // Loop over boards
    let mut boards = Vec::new();
    for chunk in lines.collect::<Vec<_>>().iter().as_slice().chunks(6) {
        let rows = chunk.iter()
            .take(5)
            .map(|line| line.split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        boards.push(Array2D::from_rows(&rows).unwrap());
    }

    part1(&numbers, &boards);
    part2(&numbers, &boards);

    Ok(())
}