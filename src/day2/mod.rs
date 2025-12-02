use aoc_runner_derive::aoc;

use crate::util::parse_initial_digits_unsigned_u64;

type Output = u64;

fn is_silly_number(id: u64) -> bool {
    let formatted = id.to_string();
    let half = formatted.len() / 2;

    formatted[..half] == formatted[half..]
}

#[aoc(day2, part1)]
fn part_one(input: &str) -> Output {
    let input = input.as_bytes();
    let mut cursor = 0;

    let mut id_sum = 0;

    while cursor < input.len() {
        let (range_start, start_digits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += start_digits + 1;
        let (range_end, end_digits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += end_digits + 1;
        for i in range_start..=range_end {
            if is_silly_number(i) {
                id_sum += i;
            }
        }
    }

    id_sum
}

pub fn part1(puzzle: &str) -> Output {
    part_one(puzzle)
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = part1(include_str!("test.txt"));
        assert_eq!(res, 1227775554);
    }
}
