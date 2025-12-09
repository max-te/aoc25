use aoc_runner_derive::aoc;

use crate::util::{parse_initial_digits, parse_initial_digits_unsigned_u64};

type Output = u64;

#[aoc(day9, part1)]
fn part_one(input: &str) -> Output {
    let input = input.as_bytes();

    // Parse
    let mut positions = Vec::with_capacity(500);
    let mut cursor = 0;
    while cursor < input.len() {
        let (x, xdigits) = parse_initial_digits(&input[cursor..]);
        cursor += xdigits + 1;
        let (y, ydigits) = parse_initial_digits(&input[cursor..]);
        cursor += ydigits + 1;
        positions.push((x, y));
    }

    // Find largest rectangle
    let mut max_size = 1;
    for (idx, corner1) in positions.iter().enumerate() {
        for corner2 in positions.iter().skip(idx + 1) {
            let size = (corner1.0.abs_diff(corner2.0) + 1) * (corner1.1.abs_diff(corner2.1) + 1);
            max_size = max_size.max(size);
        }
    }

    max_size
}

pub fn part1(puzzle: &str) -> Output {
    part_one(puzzle)
}

#[aoc(day9, part2)]
fn part_two(input: &str) -> u64 {
    let input = input.as_bytes();

    // Parse
    let mut positions = Vec::with_capacity(500);
    let mut cursor = 0;
    while cursor < input.len() {
        let (x, xdigits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += xdigits + 1;
        let (y, ydigits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += ydigits + 1;
        positions.push((x, y));
    }

    // Ensure we start with a horizontal edge
    if positions[0].0 == positions[1].0 {
        positions.rotate_right(1);
    }

    // Find largest rectangle
    let mut max_size = 1;
    for (idx, corner1) in positions.iter().enumerate() {
        'rectangles: for corner2 in positions.iter().skip(idx + 1) {
            let left = Output::min(corner1.0, corner2.0);
            let right = Output::max(corner1.0, corner2.0);
            let bot = Output::min(corner1.1, corner2.1);
            let top = Output::max(corner1.1, corner2.1);
            let size = (right - left + 1) * (top - bot + 1);
            if size > max_size {
                // The rectangle is contained if its midpoint is inside and no inner point is on an edge.
                let midpoint = (left.midpoint(right), top.midpoint(bot));
                if is_inside(&positions, midpoint) {
                    #[cfg(test)]
                    println!("Candidate rect {corner1:?} -- {corner2:?}");
                    #[cfg(test)]
                    println!("left={left},right={right} | bot={bot},top={top}");
                    for edge in positions.windows(2) {
                        if edge[0].1 == edge[1].1 {
                            // Edge is horizontal
                            #[cfg(test)]
                            println!("Hedge {edge:?}");
                            if edge[0].1 >= top || edge[0].1 <= bot {
                                #[cfg(test)]
                                println!("Is above/below");
                                continue;
                            }
                            if (edge[0].0 > left) && (edge[0].0 < right) {
                                continue 'rectangles;
                            }
                            if (edge[1].0 > left) && (edge[1].0 < right) {
                                continue 'rectangles;
                            }

                            if (edge[0].0 <= left) && (edge[1].0 >= right) {
                                continue 'rectangles;
                            }
                            if (edge[1].0 <= left) && (edge[0].0 >= right) {
                                continue 'rectangles;
                            }
                        } else {
                            // Edge is vertical
                            if edge[0].0 >= right || edge[0].0 <= left {
                                continue;
                            }
                            if (edge[0].1 > bot) && (edge[0].1 < top) {
                                continue 'rectangles;
                            }
                            if (edge[1].1 > bot) && (edge[1].1 < top) {
                                continue 'rectangles;
                            }

                            if (edge[0].1 <= bot) && (edge[1].1 >= top) {
                                continue 'rectangles;
                            }
                            if (edge[1].1 <= bot) && (edge[0].1 >= top) {
                                continue 'rectangles;
                            }
                        }
                    }
                    #[cfg(test)]
                    println!("Accepted");
                    max_size = size;
                }
            }
        }
    }

    max_size
}

fn is_inside(positions: &[(u64, u64)], point: (u64, u64)) -> bool {
    let mut winding_number = 0;
    #[cfg(test)]
    println!("Check if {point:?} is inside");
    for edge in positions.windows(2) {
        // Only consider horizontal edges
        if edge[0].1 == edge[1].1 {
            if edge[0].1 >= point.1 && (edge[0].0 <= point.0) != (edge[1].0 <= point.0) {
                // println!("Edge {edge:?}");
                winding_number += 1;
            }
        }
    }
    #[cfg(test)]
    println!("Winding no {winding_number}");
    winding_number % 2 == 1
}

pub fn part2(puzzle: &str) -> u64 {
    part_two(puzzle)
}

#[cfg(test)]
#[test]
fn example1() {
    let res = part_one(include_str!("test.txt"));
    assert_eq!(res, 50);
}

#[cfg(test)]
#[test]
fn example2() {
    let res = part2(include_str!("test.txt"));
    assert_eq!(res, 24);
}
