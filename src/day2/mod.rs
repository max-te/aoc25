use aoc_runner_derive::aoc;

use crate::util::parse_initial_digits_unsigned_u64;

type Output = u64;

fn is_silly_number(id: u64, digits: u32) -> bool {
    if digits.is_multiple_of(2) {
        let divisor = 10u64.pow(digits / 2);
        let left = id / divisor;
        let right = id % divisor;

        left == right
    } else {
        false
    }
}

#[cfg(test)]
#[test]
fn is_silly_number_test() {
    assert!(is_silly_number(11, 2));
    assert!(is_silly_number(22, 2));
    assert!(is_silly_number(1188511885, 10));
    assert!(is_silly_number(21212121, 8));
    assert!(!is_silly_number(555, 3));
    assert!(!is_silly_number(1234, 4));
    assert!(!is_silly_number(989876, 6));
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
            let digits = i.checked_ilog10().unwrap_or_default() + 1;
            if is_silly_number(i, digits) {
                id_sum += i;
            }
        }
    }

    id_sum
}

pub fn part1(puzzle: &str) -> Output {
    part_one(puzzle)
}

fn is_sillier_number(id: u64) -> bool {
    let len = id.checked_ilog10().unwrap_or_default() + 1;

    for div in 1u32..=(len / 2) {
        if len.is_multiple_of(div) {
            let divisor = 10u64.pow(div);

            let init = id % divisor;
            let mut remaining = id;
            if loop {
                if remaining == 0 {
                    return true;
                }
                if remaining % divisor != init {
                    break false;
                }
                remaining /= divisor;
            } {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
#[test]
fn is_sillier_number_test() {
    assert!(is_sillier_number(11));
    assert!(is_sillier_number(22));
    assert!(is_sillier_number(555));
    assert!(is_sillier_number(1188511885));
    assert!(is_sillier_number(2121212121));
    assert!(!is_sillier_number(1234));
    assert!(!is_sillier_number(121234));
}

#[aoc(day2, part2)]
fn part_two(input: &str) -> Output {
    let input = input.as_bytes();
    let mut cursor = 0;

    let mut id_sum = 0;

    while cursor < input.len() {
        let (range_start, start_digits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += start_digits + 1;
        let (range_end, end_digits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += end_digits + 1;
        for i in range_start..=range_end {
            if is_sillier_number(i) {
                id_sum += i;
            }
        }
    }

    id_sum
}

pub fn part2(puzzle: &str) -> Output {
    part_two(puzzle)
}

#[cfg(test)]
#[test]
fn example1() {
    let res = part1(include_str!("test.txt"));
    assert_eq!(res, 1227775554);
}

#[cfg(test)]
#[test]
fn example2() {
    let res = part2(include_str!("test.txt"));
    assert_eq!(res, 4174379265);
}
