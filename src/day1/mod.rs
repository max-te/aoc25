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
        let (mut amount, amount_digits) = parse_initial_digits(input);
        input = &input[amount_digits + 1..];
        let started_at_zero = position == 0;
        if amount >= 100 {
            zeroes += (amount / 100) as u64;
            amount %= 100;
        }

        position += match direction {
            b'L' => -amount,
            b'R' => amount,
            _ => unreachable!(),
        };

        if !started_at_zero && (position <= 0 || position >= 100) {
            zeroes += 1;
        }
        // println!(
        //     "{line} = {direction} {amount} -> {position} {zeroes} {}",
        //     position.rem_euclid(100)
        // );
        position = position.rem_euclid(100);
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
