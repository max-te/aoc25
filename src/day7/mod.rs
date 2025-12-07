use aoc_runner_derive::aoc;

use crate::util::first_line_length;

type Output = u64;

#[aoc(day7, part1)]
fn part_one(input: &str) -> Output {
    let input = input.as_bytes();
    let line_len = first_line_length(input) + 1;
    let line_count = input.len().div_ceil(line_len);
    let start = input.iter().position(|&c| c == b'S').unwrap();
    let mut beam_ends = Vec::with_capacity(line_len);
    beam_ends.push(start);

    let mut split_count = 0;
    for line_no in (2..line_count).step_by(2) {
        let mut new_beam_ends = Vec::with_capacity(line_len);
        let line = &input[line_no * line_len..line_no * line_len + line_len - 1];
        for &beam_pos in beam_ends.iter() {
            match line[beam_pos] {
                b'^' => {
                    split_count += 1;
                    if new_beam_ends.last() != Some(&(beam_pos - 1)) {
                        // Deduplication
                        new_beam_ends.push(beam_pos - 1);
                    }
                    new_beam_ends.push(beam_pos + 1);
                }
                b'.' => {
                    if new_beam_ends.last() != Some(&(beam_pos)) {
                        new_beam_ends.push(beam_pos);
                    }
                }
                _ => unreachable!(),
            }
        }
        beam_ends = new_beam_ends;
    }

    split_count
}

pub fn part1(puzzle: &str) -> Output {
    part_one(puzzle)
}

#[aoc(day7, part2)]
fn part_two(input: &str) -> Output {
    let input = input.as_bytes();
    let line_len = first_line_length(input) + 1;
    let line_count = input.len().div_ceil(line_len);
    let start = input.iter().position(|&c| c == b'S').unwrap();
    let mut beam_ends = Vec::with_capacity(line_len);
    beam_ends.push((start, 1));

    for line_no in (2..line_count).step_by(2) {
        let mut new_beam_ends: Vec<(usize, u64)> = Vec::with_capacity(line_len);
        let line = &input[line_no * line_len..line_no * line_len + line_len - 1];
        for &(beam_pos, path_count) in beam_ends.iter() {
            match line[beam_pos] {
                b'^' => {
                    let left = beam_pos - 1;
                    match new_beam_ends.last_mut() {
                        Some(prev) if prev.0 == left => {
                            prev.1 += path_count;
                        }
                        _ => {
                            new_beam_ends.push((left, path_count));
                        }
                    }
                    new_beam_ends.push((beam_pos + 1, path_count));
                }
                b'.' => match new_beam_ends.last_mut() {
                    Some(prev) if prev.0 == beam_pos => {
                        prev.1 += path_count;
                    }
                    _ => {
                        new_beam_ends.push((beam_pos, path_count));
                    }
                },
                _ => unreachable!(),
            }
        }
        beam_ends = new_beam_ends;
    }

    beam_ends.into_iter().map(|(_, count)| count).sum()
}

pub fn part2(puzzle: &str) -> Output {
    part_two(puzzle)
}

#[cfg(test)]
#[test]
fn example1() {
    let res = part1(include_str!("test.txt"));
    assert_eq!(res, 21);
}

#[cfg(test)]
#[test]
fn example2() {
    let res = part2(include_str!("test.txt"));
    assert_eq!(res, 40);
}
