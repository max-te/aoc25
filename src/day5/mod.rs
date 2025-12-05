use aoc_runner_derive::aoc;

use crate::util::parse_initial_digits_unsigned_u64;

type Output = u64;

#[aoc(day5, part1)]
fn part_one(input: &str) -> Output {
    let input = input.as_bytes();
    let mut ranges = Vec::new();
    let mut cursor = 0;

    while input[cursor] != b'\n' {
        let (start, start_digits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += start_digits + 1;
        let (end, end_digits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += end_digits;
        cursor += 1;
        ranges.push(start..=end);
    }

    let mut good_ingredients = 0;
    while cursor < input.len() {
        let (number, digits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += digits + 1;
        if ranges.iter().any(|r| r.contains(&number)) {
            good_ingredients += 1;
        }
    }

    good_ingredients
}

pub fn part1(puzzle: &str) -> Output {
    part_one(puzzle)
}

#[aoc(day5, part2)]
fn part_two(input: &str) -> Output {
    let input = input.as_bytes();
    let mut ranges = Vec::new();
    let mut cursor = 0;

    while input[cursor] != b'\n' {
        let (start, start_digits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += start_digits + 1;
        let (end, end_digits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += end_digits;
        cursor += 1;
        ranges.push((start, end));
    }
    ranges.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut last = ranges[0];
    let mut total_len = 0;
    for range in &ranges[1..] {
        if range.0 <= last.1 {
            last.1 = last.1.max(range.1);
        } else {
            total_len += last.1 - last.0 + 1;
            last = *range;
        }
    }
    total_len += last.1 - last.0 + 1;

    total_len
}

pub fn part2(puzzle: &str) -> Output {
    part_two(puzzle)
}

#[cfg(test)]
#[test]
fn example1() {
    let res = part1(include_str!("test.txt"));
    assert_eq!(res, 3);
}

#[cfg(test)]
#[test]
fn example2() {
    let res = part2(include_str!("test.txt"));
    assert_eq!(res, 14);
}
