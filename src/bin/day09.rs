use array2d::Array2D;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fs;

fn part1(height_map: &Array2D<i8>) {
    let mut risk = 0;
    for row in 1..height_map.num_rows() - 1 {
        for col in 1..height_map.num_columns() - 1 {
            let val = height_map[(row, col)];
            if height_map[(row + 1, col)] > val && height_map[(row - 1, col)] > val &&
                height_map[(row, col - 1)] > val && height_map[(row, col + 1)] > val {
                risk += val as u64 + 1;
            }
        }
    }

    println!("Part 1: {}", risk);
}


fn flood_count(image: &mut Array2D<i8>, start: (usize, usize)) -> u64 {
    let rows = image.num_rows() - 1;
    let cols = image.num_columns() - 1;

    if start.0 >= rows || start.1 >= cols || image[start] == 9 {
        return 0;
    }

    let mut count = 1;
    image[start] = 9;

    if start.0 > 1 {
        count += flood_count(image, (start.0 - 1, start.1));
    }
    if start.0 + 1 < rows {
        count += flood_count(image, (start.0 + 1, start.1));
    }
    if start.1 > 1 {
        count += flood_count(image, (start.0, start.1 - 1));
    }
    if start.1 + 1 < cols {
        count += flood_count(image, (start.0, start.1 + 1));
    }

    count
}


fn part2(height_map: &Array2D<i8>) {
    let mut mut_map = height_map.clone();

    //let mut risk = 0;
    let mut heap = BinaryHeap::with_capacity(3);
    for row in 1..height_map.num_rows() - 1 {
        for col in 1..height_map.num_columns() - 1 {
            let val = height_map[(row, col)];
            if height_map[(row + 1, col)] > val && height_map[(row - 1, col)] > val &&
                height_map[(row, col - 1)] > val && height_map[(row, col + 1)] > val {

                // Calculate basin size (sets all basin elements to 9)
                let basin_size = flood_count(&mut mut_map, (row, col));
                
                // Keep 3 largest values
                if heap.len() < 3 {
                    heap.push(Reverse(basin_size));
                } else if let Some(&Reverse(min)) = heap.peek() {
                    if basin_size > min {
                        heap.pop();
                        heap.push(Reverse(basin_size));
                    }
                }
            }
        }
    }

    let mut risk = 1;
    for val in heap {
        risk *= val.0;
    }
    println!("Part 2: {}", risk);

}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day09/day09.txt")?;

    let mut rows = Vec::new();
    for line in file_str.lines() {
        rows.push(line.bytes().map(|b| b - b'0').collect::<Vec<_>>());
    }

    let input_map = Array2D::from_rows(&rows).unwrap();
    let mut height_map =
        Array2D::filled_by_row_major(|| i8::MAX, input_map.num_rows() + 2, input_map.num_columns() + 2);
    for (row, col) in input_map.indices_row_major() {
        height_map[(row + 1, col + 1)] = input_map[(row, col)] as i8;
    }

    //println!("{:?}", height_map);
    part1(&height_map);
    part2(&height_map);

    Ok(())
}