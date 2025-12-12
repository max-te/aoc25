use aoc_runner_derive::aoc;

use crate::util::parse_initial_digits_unsigned_u64;

type Output = usize;
type Shape = [bool; 9];

#[aoc(day12, part1)]
fn part_one(input: &str) -> Output {
    let input = input.as_bytes();
    let mut cursor = 0;

    let mut shapes = Vec::new();
    while input[cursor + 1] == b':' {
        cursor += 3; // N:\n
        let mut shape: Shape = [false; 9];
        for i in 0..3 {
            for j in 0..3 {
                shape[i * 3 + j] = input[cursor + i * 4 + j] == b'#';
            }
        }
        shapes.push(shape);

        cursor += 3 * 4 + 1;
    }

    let mut admissable_puzzles = 0;
    while cursor < input.len() {
        let (w, w_digits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += w_digits + 1;
        let (h, h_digits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += h_digits + 2;
        let area = w * h;

        let mut shape_idx = 0;
        let mut required_area = 0;
        let mut total_shapes_count = 0;
        while shape_idx < shapes.len() {
            let (shape_count, shape_count_digits) =
                parse_initial_digits_unsigned_u64(&input[cursor..]);
            cursor += shape_count_digits + 1;
            total_shapes_count += shape_count;
            let shape = shapes[shape_idx];
            let shape_size: u64 = shape.iter().map(|&b| if b { 1 } else { 0 }).sum();
            required_area += shape_size * shape_count;
            shape_idx += 1;
        }
        if total_shapes_count * 9 <= area {
            admissable_puzzles += 1;
        } else if required_area <= area {
            todo!("Hard puzzle at {cursor}");
        }
    }
    admissable_puzzles
}

pub fn part1(puzzle: &str) -> Output {
    part_one(puzzle)
}

#[aoc(day12, part2)]
fn part_two(_input: &str) -> Output {
    0
}

pub fn part2(puzzle: &str) -> Output {
    part_two(puzzle)
}

// #[cfg(test)]
// #[test]
// fn example1() {
//     let res = part_one(include_str!("test.txt"));
//     assert_eq!(res, 2);
// }
