use array2d::Array2D;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::fs;


#[derive(Debug, Clone, Eq, PartialEq)]
struct CostLocation {
    cost: u64,
    location: (usize, usize),
}

impl Ord for CostLocation {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse order for min-heap
    }
}

impl PartialOrd for CostLocation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> u64 {
    let dx = (a.0 as i32 - b.0 as i32).abs() as u64;
    let dy = (a.1 as i32 - b.1 as i32).abs() as u64;
    dx + dy
}

fn explore(map: &Array2D<u8>,
           start_location: (usize, usize),
           end_location: (usize, usize)) -> u64 {
    let moves = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
    ];
    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();

    distances.insert(start_location, 0);
    heap.push(CostLocation {
        cost: manhattan_distance(start_location, end_location),
        location: start_location,
    });

    while let Some(CostLocation { cost: _, location: current_location }) = heap.pop() {
        if current_location == end_location {
            return distances[&current_location];
        }

        for m in moves.iter() {
            let new_row = current_location.0 as i32 + m.0;
            let new_col = current_location.1 as i32 + m.1;

            if new_row < 0 || new_col < 0 ||
                new_row >= map.num_rows() as i32 || new_col >= map.num_columns() as i32 {
                continue;
            }

            let new_location = (new_row as usize, new_col as usize);
            let new_cost = distances[&current_location] + map[new_location] as u64;
            if new_cost < *distances.get(&new_location).unwrap_or(&u64::MAX) {
                distances.insert(new_location, new_cost);
                heap.push(CostLocation {
                    cost: new_cost + manhattan_distance(new_location, end_location),
                    location: new_location,
                });
            }
        }
    }

    u64::MAX
}


fn part1(map: &Array2D<u8>) {
    let start = (0, 0);
    let end = (map.num_rows() - 1, map.num_columns() - 1);

    let min_cost = explore(map, start, end);
    println!("Part 1: {}", min_cost);
}

/*
fn print_levels(energy_levels: &Array2D<u8>) {
    for row in 0..energy_levels.num_rows() {
        for col in 0..energy_levels.num_columns() {
            print!("{}", energy_levels[(row, col)]);
        }
        println!();
    }
}
 */

fn part2(map: &Array2D<u8>) {
    let num_rows = map.num_rows();
    let num_cols = map.num_columns();
    let mut expanded_map =
        Array2D::filled_by_row_major(|| 0, 5 * num_rows, 5 * num_cols);

    for ((row, col), val) in map.enumerate_row_major() {
        for row_block in 0..5 {
            for col_block in 0..5 {
                expanded_map[(row + row_block * num_rows, col + col_block * num_cols)] =
                    (val + row_block as u8 + col_block as u8 - 1) % 9 + 1;
            }
        }
    }

    let start = (0, 0);
    let end = (expanded_map.num_rows() - 1, expanded_map.num_columns() - 1);

    let min_cost = explore(&expanded_map, start, end);
    println!("Part 2: {}", min_cost);
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day15/day15.txt")?;

    let mut rows = Vec::new();
    for line in file_str.lines() {
        rows.push(line.bytes().map(|b| b - b'0').collect::<Vec<_>>());
    }

    let map = Array2D::from_rows(&rows).unwrap();

    part1(&map);
    part2(&map);

    Ok(())
}