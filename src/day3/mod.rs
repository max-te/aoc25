use aoc_runner_derive::aoc;

use crate::util::first_line_length;

type Output = u64;

#[aoc(day3, part1)]
fn part_one(input: &str) -> Output {
    let input = input.as_bytes();
    let mut cursor = 0;

    let mut joltage_sum = 0;

    while cursor < input.len() {
        let mut highest_before = 0;
        let mut highest = 0;
        let mut highest_after = 0;

        while cursor < input.len() && input[cursor] != b'\n' {
            let val = input[cursor] - b'0';
            if val > highest {
                highest_before = highest;
                highest = val;
                highest_after = 0;
            } else if val > highest_after {
                highest_after = val;
            }
            cursor += 1;
        }

        let block_joltage = if highest_after > 0 {
            highest * 10 + highest_after
        } else {
            highest_before * 10 + highest
        };
        joltage_sum += block_joltage as u64;

        cursor += 1;
    }

    joltage_sum
}

pub fn part1(puzzle: &str) -> Output {
    part_one(puzzle)
}

#[aoc(day3, part2)]
fn part_two(input: &str) -> Output {
    let input = input.as_bytes();
    let mut cursor = 0;
    let mut joltage_total = 0;
    let line_length = first_line_length(&input);

    while cursor < input.len() {
        let block_joltage = find_block_joltage(&input[cursor..cursor + line_length], 12);
        joltage_total += block_joltage;
        cursor += 1 + line_length;
    }

    joltage_total
}

fn find_block_joltage(block: &[u8], digits: usize) -> Output {
    if digits == 0 {
        return 0;
    }

    let mut max_pos = 0;
    let mut max = 0;
    for pos in 0..block.len() {
        let val = block[pos];
        if val > max {
            max_pos = pos;
            max = val;
        }
    }
    let max = (max - b'0') as u64;

    let left = &block[..max_pos];
    let right = &block[max_pos + 1..];

    let remaining_digits = digits - 1;
    let right_digits = remaining_digits.min(right.len());
    let left_digits = remaining_digits - right_digits;
    let left_value = find_block_joltage(left, left_digits);
    let right_value = find_block_joltage(right, right_digits);

    (left_value * 10 + max) * 10u64.pow(right_digits as u32) + right_value
}

#[cfg(test)]
#[test]
fn test_find_block_joltage() {
    assert!(find_block_joltage(b"987654321111111", 12) == 987654321111);
    assert!(find_block_joltage(b"987654321111111", 2) == 98);
    assert!(find_block_joltage(b"811111111111119", 12) == 811111111119);
    assert!(find_block_joltage(b"811111111111119", 2) == 89);
    assert!(find_block_joltage(b"234234234234278", 12) == 434234234278);
    assert!(find_block_joltage(b"818181911112111", 12) == 888911112111);
}

pub fn part2(puzzle: &str) -> Output {
    part_two(puzzle)
}

#[cfg(test)]
#[test]
fn example1() {
    let res = part1(include_str!("test.txt"));
    assert_eq!(res, 357);
}

#[cfg(test)]
#[test]
fn example2() {
    let res = part2(include_str!("test.txt"));
    assert_eq!(res, 3121910778619);
}
