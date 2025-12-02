use aoc_runner_derive::aoc;

use crate::util::parse_initial_digits_unsigned_u64;

type Output = u64;

fn is_silly_number(id: u64, digits: u32) -> bool {
    if digits.is_multiple_of(2) {
        let divisor = 10u64.pow(digits / 2);
        let left = id / divisor;
        let expected = left * divisor + left;
        id == expected
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

fn sum_2_sillies(range_start: u64, range_end: u64, start_digits: u32) -> Output {
    let mut sum = 0;
    let mut half_digits = start_digits.div_ceil(2);

    let mut silly_step = 10u64.pow(half_digits) + 1;
    let mut silly = if !start_digits.is_multiple_of(2) {
        10u64.pow(half_digits * 2 - 1) + 10u64.pow(half_digits - 1)
    } else {
        let divisor = 10u64.pow(half_digits);
        let left = range_start / divisor;
        let silly = left + left * divisor;
        if silly >= range_start {
            silly
        } else {
            silly + silly_step
        }
    };

    let mut digit_increment = silly_step * 10u64.pow(half_digits);
    #[cfg(test)]
    println!(
        "Next silly from {range_start} is {silly} with {half_digits}*2 digits. Go in {silly_step}s until {digit_increment}"
    );
    while silly <= range_end {
        if silly == digit_increment {
            half_digits += 1;
            silly_step = 10u64.pow(half_digits) + 1;
            silly = 10u64.pow(half_digits * 2 - 1) + 10u64.pow(half_digits - 1);
            digit_increment = silly_step * 10u64.pow(half_digits);
            #[cfg(test)]
            println!(
                "Silly jump to {silly} with {half_digits}*2 digits. Go in {silly_step}s until {digit_increment}"
            );
        } else {
            sum += silly;
            silly += silly_step;
            #[cfg(test)]
            println!("Next {silly}")
        }
    }
    #[cfg(test)]
    println!("reached end {range_end}");
    sum
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

        id_sum += sum_2_sillies(range_start, range_end, start_digits as u32);
    }

    id_sum
}

pub fn part1(puzzle: &str) -> Output {
    part_one(puzzle)
}

static PRIMES: &[u32] = &[3, 5, 7]; // u32::MAX has 10 digits, this should suffice

fn is_sillier_number(id: u64, len: u32) -> bool {
    for &num_parts in PRIMES {
        if len.is_multiple_of(num_parts) {
            let part_len = len / num_parts;
            let divisor = 10u64.pow(part_len);

            let init = id % divisor;
            let mut expected = init;
            for _ in 1..num_parts {
                expected *= divisor;
                expected += init
            }

            if id == expected {
                if is_silly_number(id, len) {
                    return false;
                }
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
#[test]
fn is_sillier_number_test() {
    assert!(!is_sillier_number(11, 2));
    assert!(!is_sillier_number(22, 2));
    assert!(is_sillier_number(555, 3));
    assert!(!is_sillier_number(1188511885, 10));
    assert!(is_sillier_number(2121212121, 10));
    assert!(!is_sillier_number(1234, 4));
    assert!(!is_sillier_number(121234, 6));
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
        let mut digits = start_digits as u32;
        let mut digit_increment = 10u64.pow(digits);

        id_sum += sum_2_sillies(range_start, range_end, digits);
        for i in range_start..=range_end {
            if i == digit_increment {
                digit_increment *= 10;
                digits += 1;
                // Powers of 10 are not silly
            } else if is_sillier_number(i, digits) {
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
