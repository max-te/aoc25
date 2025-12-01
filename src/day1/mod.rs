use aoc_runner_derive::aoc;

use crate::util::parse_initial_digits_unsigned_i16;

type Output = u16;

#[aoc(day1, part1)]
fn part_one(input: &str) -> Output {
    let input = input.as_bytes();
    let mut cursor = 0;

    let mut position = 50;
    let mut zeroes = 0;

    while cursor < input.len() {
        let direction = input[cursor];
        cursor += 1;
        let (amount, amount_digits) = parse_initial_digits_unsigned_i16(&input[cursor..]);
        cursor += amount_digits + 1;
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
    let input = input.as_bytes();
    let mut cursor = 0;

    let mut position = 50;
    let mut zeroes = 0;

    while cursor < input.len() {
        let direction = input[cursor];
        cursor += 1;
        let (amount, amount_digits) = parse_initial_digits_unsigned_i16(&input[cursor..]);
        cursor += amount_digits + 1;
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
