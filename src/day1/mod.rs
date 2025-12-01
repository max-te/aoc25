use aoc_runner_derive::aoc;

use crate::util::parse_initial_digits;

type Output = u64;

#[aoc(day1, part1)]
fn part_one(input: &str) -> Output {
    let mut input = input.as_bytes();

    let mut position = 50;
    let mut zeroes = 0;

    while !input.is_empty() {
        let direction = input[0];
        input = &input[1..];
        let (amount, amount_digits) = parse_initial_digits(input);
        input = &input[amount_digits + 1..];
        position = (position
            + match direction {
                b'L' => -amount,
                b'R' => amount,
                _ => unreachable!(),
            })
            % 100;
        if position == 0 {
            zeroes += 1;
        }
    }
    zeroes
}

pub fn part1(puzzle: &str) -> Output {
    part_one(puzzle)
}

#[aoc(day1, part2)]
fn part_two(input: &str) -> Output {
    let mut input = input.as_bytes();

    let mut position = 50;
    let mut zeroes = 0;

    while !input.is_empty() {
        let direction = input[0];
        input = &input[1..];
        let (amount, amount_digits) = parse_initial_digits(input);
        input = &input[amount_digits + 1..];
        let not_started_at_zero = if position == 0 { 0 } else { 1 };

        position += match direction {
            b'L' => -amount,
            b'R' => amount,
            _ => unreachable!(),
        };

        if position <= 0 {
            zeroes += (position.unsigned_abs() / 100) + not_started_at_zero;
            position = position.rem_euclid(100);
        } else if position >= 100 {
            zeroes += position.unsigned_abs() / 100;
            position = position.rem_euclid(100);
        }
    }
    zeroes
}

pub fn part2(puzzle: &str) -> Output {
    part_two(puzzle)
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = part1(include_str!("test.txt"));
        assert_eq!(res, 3);
    }

    #[test]
    fn example2() {
        let res = part2(include_str!("test.txt"));
        assert_eq!(res, 6);
    }
}
