use std::mem;

use aoc_runner_derive::aoc;

use crate::util::first_line_length;

type Output = u64;

#[aoc(day4, part1)]
fn part_one(input: &str) -> Output {
    let input = input.as_bytes();
    let width = first_line_length(input);

    let neighbor_offsets: [isize; 8] = [
        -(width as isize) - 2,
        -(width as isize) - 1,
        -(width as isize),
        1,
        (width as isize) + 2,
        (width as isize) + 1,
        width as isize,
        -1,
    ];

    let mut free_count = 0;

    let mut cursor = 0;
    while cursor < input.len() {
        if input[cursor] != b'@' {
            cursor += 1;
            continue;
        }
        let mut occupied_neighbors = 0;
        for offset in neighbor_offsets {
            let ncursor = cursor.wrapping_add_signed(offset);
            if ncursor < input.len() && input[ncursor as usize] == b'@' {
                occupied_neighbors += 1
            }
        }
        if occupied_neighbors < 4 {
            free_count += 1;
        }
        cursor += 1;
    }
    free_count
}

pub fn part1(puzzle: &str) -> Output {
    part_one(puzzle)
}

#[aoc(day4, part2)]
fn part_two(input: &str) -> Output {
    let mut input = input.as_bytes().to_vec();
    let width = first_line_length(&input);

    let neighbor_offsets: [isize; 8] = [
        -(width as isize) - 2,
        -(width as isize) - 1,
        -(width as isize),
        1,
        (width as isize) + 2,
        (width as isize) + 1,
        width as isize,
        -1,
    ];

    let mut free_count = 0;
    let mut positions = Vec::with_capacity(input.len());

    for cursor in 0..input.len() {
        if input[cursor] != b'@' {
            continue;
        }
        let mut occupied_neighbors = 0;
        for offset in neighbor_offsets {
            let ncursor = cursor.wrapping_add_signed(offset);
            if ncursor < input.len() && input[ncursor as usize] == b'@' {
                occupied_neighbors += 1
            }
        }
        if occupied_neighbors < 4 {
            input[cursor] = b'.';
            free_count += 1;
        } else {
            positions.push(cursor);
        }
    }

    let mut new_positions = Vec::with_capacity(positions.len());
    loop {
        new_positions.clear();
        let mut freed_this_round = 0;
        for &cursor in &positions {
            let mut occupied_neighbors = 0;
            for offset in neighbor_offsets {
                let ncursor = cursor.wrapping_add_signed(offset);
                if ncursor < input.len() && input[ncursor as usize] == b'@' {
                    occupied_neighbors += 1
                }
            }
            if occupied_neighbors < 4 {
                input[cursor] = b'.';
                freed_this_round += 1;
            } else {
                new_positions.push(cursor);
            }
        }

        if freed_this_round > 0 {
            free_count += freed_this_round;
        } else {
            break;
        }
        mem::swap(&mut positions, &mut new_positions);
    }

    free_count
}

pub fn part2(puzzle: &str) -> Output {
    part_two(puzzle)
}

#[cfg(test)]
#[test]
fn example1() {
    let res = part1(include_str!("test.txt"));
    assert_eq!(res, 13);
}

#[cfg(test)]
#[test]
fn example2() {
    let res = part2(include_str!("test.txt"));
    assert_eq!(res, 43);
}
