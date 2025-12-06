use aoc_runner_derive::aoc;

use crate::util::{first_line_length, parse_initial_digits_unsigned_u64};

type Output = u64;

#[aoc(day6, part1)]
fn part_one(input: &str) -> Output {
    let input = input.as_bytes();
    let line_len = first_line_length(input) + 1;
    let line_count = input.len().div_ceil(line_len);
    let mut cursor = 0;
    let mut total = 0;

    while cursor < line_len - 1 {
        let operand_line = &input[line_len * (line_count - 1) + cursor..];
        let operand = operand_line[0];
        let width = operand_line[1..]
            .iter()
            .position(|c| *c != b' ')
            .unwrap_or(operand_line.len() - 1)
            + 1;

        let (op, identity): (fn(Output, Output) -> Output, Output) = match operand {
            b'+' => (Output::wrapping_add, 0),
            b'*' => (Output::wrapping_mul, 1),
            _ => panic!(
                "Unexpected operand {:?} at position {cursor} ({})",
                char::from_u32(operand as u32),
                line_len * (line_count - 1) + cursor
            ),
        };
        let mut col_result = identity;
        for line_no in 0..(line_count - 1) {
            let number = parse_initial_digits_unsigned_u64(
                input[line_len * line_no + cursor..line_len * line_no + cursor + width]
                    .trim_ascii_start(),
            )
            .0;
            col_result = op(col_result, number);
        }
        total += col_result;

        cursor += width;
    }

    total
}

pub fn part1(puzzle: &str) -> Output {
    part_one(puzzle)
}

#[aoc(day6, part2)]
fn part_two(input: &str) -> Output {
    let input = input.as_bytes();
    let line_len = first_line_length(input) + 1;
    let line_count = input.len().div_ceil(line_len);
    let mut cursor = 0;
    let mut total = 0;

    while cursor < line_len - 1 {
        let operand_line = &input[line_len * (line_count - 1) + cursor..];
        let operand = operand_line[0];
        let mut width = operand_line[1..]
            .iter()
            .position(|c| *c != b' ')
            .unwrap_or(operand_line.len() - 1)
            + 1;
        if width == operand_line.len() || operand_line[width] == b'\n' {
            width += 1;
        }

        let (op, identity): (fn(Output, Output) -> Output, Output) = match operand {
            b'+' => (Output::wrapping_add, 0),
            b'*' => (Output::wrapping_mul, 1),
            _ => panic!(
                "Unexpected operand {:?} at position {cursor} ({})",
                char::from_u32(operand as u32),
                line_len * (line_count - 1) + cursor
            ),
        };

        let mut calc_result = identity;
        for col_no in 0..(width - 1) {
            let mut number = 0;
            for line_no in 0..(line_count - 1) {
                let cell_pos = line_len * line_no + cursor + col_no;
                let chr = input[cell_pos];
                if chr == b' ' {
                    continue;
                }
                let digit = chr - b'0';
                number = number * 10 + digit as Output;
            }
            calc_result = op(calc_result, number);
        }
        total += calc_result;

        cursor += width;
    }

    total
}

pub fn part2(puzzle: &str) -> Output {
    part_two(puzzle)
}

#[cfg(test)]
#[test]
fn example1() {
    let res = part1(include_str!("test.txt"));
    assert_eq!(res, 4277556);
}

#[cfg(test)]
#[test]
fn example2() {
    let res = part2(include_str!("test.txt"));
    assert_eq!(res, 3263827);
}
