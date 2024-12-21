use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
struct Transform {
    index: usize,
    sign: i64,
    offset: i64,
}

fn distances(scanner1: &Vec<Vec<i64>>, scanner2: &Vec<Vec<i64>>) -> Option<[Transform; 3]> {

    // Compare against x-axis of the first scanner
    let indices = HashSet::from([0, 1, 2]);
    let signs = [-1, 1];

    let mut x_transform = None;
    let mut x_matches = None;

    'outer: for index2 in indices.iter() {
        for sign in signs.iter() {
            let mut entries = Vec::new();
            let mut diffs = HashMap::new();

            for idx1 in 0..scanner1.len() {
                for idx2 in 0..scanner2.len() {
                    let diff = scanner1[idx1][0] - sign * scanner2[idx2][*index2];
                    *diffs.entry(diff).or_insert(0) += 1;
                    entries.push((diff, idx1, idx2));
                }
            }

            if let Some((diff, max_count)) = diffs.iter().max_by_key(|&(_, v)| v) {
                if *max_count >= 12 {
                    x_transform = Some(Transform {
                        index: *index2,
                        sign: *sign,
                        offset: *diff,
                    });

                    let matches = entries.iter()
                        .filter(|&(d, _, _)| d == diff)
                        .map(|&(_, i1, i2)| (i1, i2))
                        .collect::<Vec<_>>();

                    x_matches = Some(matches);
                    break 'outer;
                }
            }
        }
    }

    if let Some(some_x_transform) = x_transform {
        if let Some(matches) = x_matches {

            // Find transform for y
            let mut y_transform = None;
            let mut indices = indices.clone();
            indices.remove(&some_x_transform.index);

            'outer: for index2 in indices.iter() {
                for sign in signs.iter() {
                    let mut count = 0;
                    let val = scanner1[matches[0].0][1] - sign * scanner2[matches[0].1][*index2];
                    for pair in matches.iter() {
                        if scanner1[pair.0][1] - sign * scanner2[pair.1][*index2] == val {
                            count += 1;
                        }
                    }

                    if count >= 12 {
                        y_transform = Some(Transform {
                            index: *index2,
                            sign: *sign,
                            offset: val,
                        });
                        break 'outer;
                    }
                }
            }

            if let Some(some_y_transform) = y_transform {
                // Find transform for z
                let mut z_transform = None;
                indices.remove(&some_y_transform.index);

                'outer: for index2 in indices.iter() {
                    for sign in signs.iter() {
                        let mut count = 0;
                        let val = scanner1[matches[0].0][2] - sign * scanner2[matches[0].1][*index2];
                        for pair in matches.iter() {
                            if scanner1[pair.0][2] - sign * scanner2[pair.1][*index2] == val {
                                count += 1;
                            }
                        }

                        if count >= 12 {
                            z_transform = Some(Transform {
                                index: *index2,
                                sign: *sign,
                                offset: val,
                            });
                            break 'outer;
                        }
                    }
                }

                if let Some(some_z_transform) = z_transform {
                    return Some([some_x_transform, some_y_transform, some_z_transform]);
                }
            }
        }
    }

    None
}


fn transform(coord: &[i64], transform: &[Transform]) -> Vec<i64> {
    let mut xform_coord = Vec::new();

    for i in 0..3 {
        xform_coord.push(transform[i].offset + transform[i].sign * coord[transform[i].index]);
    }

    xform_coord
}


fn inverse_transform(coord: &[i64], transform: &[Transform]) -> Vec<i64> {
    let mut xform_coord = Vec::from(coord);

    for i in 0..3 {
        xform_coord[transform[i].index] = transform[i].sign * (coord[i] - transform[i].offset);
    }

    xform_coord
}

fn find_transform(current: usize,
                  target: usize,
                  scanner: &Vec<Vec<i64>>,
                  transforms: &[(usize, usize, [Transform; 3])]) -> Option<Vec<Vec<i64>>> {
    if current == target {
        return Some(scanner.clone());
    }

    let mut stack = vec![(current, scanner.clone())];
    let mut visited = HashSet::new();
    visited.insert(current);

    while let Some((curr, scan)) = stack.pop() {
        if curr == target {
            return Some(scan);
        }

        for tf in transforms.iter() {
            if tf.0 == curr && !visited.contains(&tf.1) {
                let xform = scan.iter()
                    .map(|c| transform(c, &tf.2))
                    .collect::<Vec<_>>();
                stack.push((tf.1, xform));
                visited.insert(tf.1);
            } else if tf.1 == curr && !visited.contains(&tf.0) {
                let xform = scan.iter()
                    .map(|c| inverse_transform(c, &tf.2))
                    .collect::<Vec<_>>();
                stack.push((tf.0, xform));
                visited.insert(tf.0);
            }
        }
    }

    None
}


fn part1(scanners: &Vec<Vec<Vec<i64>>>) {
    // Get transforms between neighbours
    let mut transforms = Vec::new();
    for idx1 in 0..scanners.len() {
        for idx2 in idx1 + 1..scanners.len() {
            if let Some(xform) = distances(&scanners[idx1], &scanners[idx2]) {
                transforms.push((idx2, idx1, xform));
            }
        }
    }

    // Transform beacons into scanner 0 coordinates
    let mut beacons = HashSet::new();
    beacons.extend(scanners[0].clone());
    for from in 1..scanners.len() {
        if let Some(xformed) = find_transform(from, 0, &scanners[from], &transforms) {
            beacons.extend(xformed.clone());
        }
    }

    println!("Part 1: {}", beacons.len());
}

fn part2(scanners: &Vec<Vec<Vec<i64>>>) {
    // Get transforms between neighbours
    let mut transforms = Vec::new();
    for idx1 in 0..scanners.len() {
        for idx2 in idx1 + 1..scanners.len() {
            if let Some(xform) = distances(&scanners[idx1], &scanners[idx2]) {
                transforms.push((idx2, idx1, xform));
            }
        }
    }

    // Map origin of scanners into scanner 0
    let origin = (&[&[0, 0, 0]]).iter().map(|v| v.to_vec()).collect::<Vec<_>>();
    let mut locations = Vec::new();
    locations.push(origin[0].clone());
    for from in 1..scanners.len() {
        if let Some(xformed) = find_transform(from, 0, &origin, &transforms) {
            locations.push(xformed[0].clone());
        }
    }

    // Pair-wise distances
    let mut max_distance = 0;
    for idx1 in 0..locations.len() {
        for idx2 in idx1 + 1..scanners.len() {
            let dist = locations[idx1].iter().zip(locations[idx2].iter())
                .fold(0, |acc, (c1, c2)| acc + (c1 - c2).abs());
            max_distance = std::cmp::max(max_distance, dist);
        }
    }

    println!("Part 2: {}", max_distance);
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day19/day19.txt")?;

    let mut scanners = Vec::new();
    let mut scanner = Vec::new();
    for line in file_str.lines() {
        if line.is_empty() {
            continue;
        } else if line.starts_with("---") {
            if !scanner.is_empty() {
                scanners.push(scanner);
            }
            scanner = Vec::new();
        } else {
            let nums = line.split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            scanner.push(nums);
        }
    }
    if !scanner.is_empty() {
        scanners.push(scanner);
    }

    part1(&scanners);
    part2(&scanners);

    Ok(())
}