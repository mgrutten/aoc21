use std::cmp::max;
use std::error::Error;
use std::fs;

fn parse_number(num: &[char]) -> Vec<[u64; 2]> {
    let mut depth = 0;
    let mut vals = Vec::new();
    for idx in 0..num.len() {
        match num[idx] {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => {}
            '0'..='9' => vals.push([num[idx].to_digit(10).unwrap() as u64, depth - 1]),
            _ => unreachable!(),
        }
    }

    vals
}

fn explode(num: &Vec<[u64; 2]>) -> Option<Vec<[u64; 2]>> {
    for i in 0..num.len() {
        let depth = num[i][1];
        if depth < 4 {
            continue;
        }

        let mut num_mut = num.clone();
        if i > 0 {
            num_mut[i - 1][0] += num_mut[i][0];
        }

        if i + 2 < num_mut.len() {
            num_mut[i + 2][0] += num_mut[i + 1][0];
        }

        num_mut[i][0] = 0;
        num_mut[i][1] = depth - 1;
        num_mut.remove(i + 1);

        return Some(num_mut);
    }

    None
}

fn split(num: &Vec<[u64; 2]>) -> Option<Vec<[u64; 2]>> {
    for i in 0..num.len() {
        let val = num[i][0];
        if val < 10 {
            continue;
        }

        let (left, right) = if val % 2 == 0 {
            (val / 2, val / 2)
        } else {
            (val / 2, val / 2 + 1)
        };

        let mut num_mut = num.clone();
        num_mut[i][0] = left;
        num_mut[i][1] += 1;
        num_mut.insert(i + 1, [right, num_mut[i][1]]);

        return Some(num_mut);
    }

    None
}

fn reduce(num: &Vec<[u64; 2]>) -> Vec<[u64; 2]> {
    let mut num_mut = num.clone();
    loop {
        let mut found = false;
        if let Some(result) = explode(&num_mut) {
            num_mut = result;
            found = true;
        }
        if !found {
            if let Some(result) = split(&num_mut) {
                num_mut = result;
                found = true;
            }
        }

        if !found {
            break;
        }
    }

    num_mut
}

fn add(left: &Vec<[u64; 2]>, right: &Vec<[u64; 2]>) -> Vec<[u64; 2]> {
    let mut num = left.clone();

    num.extend(right);
    num.iter_mut().for_each(|[_, depth]| *depth += 1);

    reduce(&num)
}


fn magnitude(num: &Vec<[u64; 2]>) -> u64 {
    let mut mut_num = num.clone();
    while mut_num.len() > 1 {
        for i in 0..mut_num.len() - 1 {
            if mut_num[i][1] == mut_num[i + 1][1] {
                mut_num[i][0] = 3 * mut_num[i][0] + 2 * mut_num[i + 1][0];
                if mut_num[i][1] > 0 {
                    mut_num[i][1] -= 1;
                }
                mut_num.remove(i + 1);
                break;
            }
        }
    }

    mut_num[0][0]
}

fn part1(sailfish: &Vec<Vec<[u64; 2]>>) {
    let sum = sailfish.iter().skip(1)
        .fold(sailfish[0].clone(), |acc, num| add(&acc, num));
    
    println!("Part 1: {}", magnitude(&sum));
}


fn part2(sailfish: &Vec<Vec<[u64; 2]>>) {
    let mut max_magnitude = 0;
    for num1 in sailfish {
        for num2 in sailfish {
            max_magnitude = max(max_magnitude, magnitude(&add(num1, num2)));
        }
    }

    println!("Part 2: {}", max_magnitude);
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day18/day18.txt")?;

    let sailfish = file_str.lines()
        .map(|line| parse_number(&line.chars().collect::<Vec<_>>()))
        .collect::<Vec<Vec<_>>>();

    part1(&sailfish);
    part2(&sailfish);

    Ok(())
}