use std::ops::BitXor;

use aoc_runner_derive::aoc;
use good_lp::{Expression, ProblemVariables};
use good_lp::{Solution, SolverModel, default_solver, variable};
use pathfinding::directed::dijkstra::dijkstra;
use smallvec::SmallVec;

use crate::util::parse_initial_digits_unsigned_u32;

type Output = u64;

#[aoc(day10, part1)]
fn part_one(input: &str) -> Output {
    let input = input.as_bytes();
    let mut cursor = 0;
    let mut total = 0;

    while cursor < input.len() {
        let len = input[cursor..].iter().position(|c| *c == b']').unwrap();
        assert!(len <= 16);
        cursor += 1; // [
        let mut target_lights: u16 = 0;
        for i in (0..len).rev() {
            target_lights <<= 1;
            let is_on = input[cursor + i] == b'#';
            target_lights |= if is_on { 1 } else { 0 };
        }
        cursor += len + 1;
        #[cfg(test)]
        print!("[{target_lights:b}] ");

        let mut buttons = Vec::new();
        while input[cursor] == b'(' {
            cursor += 1;
            let mut button = 0;
            while input[cursor] != b' ' {
                let (num, digits) = parse_initial_digits_unsigned_u32(&input[cursor..]);
                // print!(" {num}");
                button |= 1 << num;
                cursor += digits + 1;
            }
            buttons.push(button);
            #[cfg(test)]
            print!(" ({button:b})");
            cursor += 1;
        }
        #[cfg(test)]
        println!("");

        let res: (_, u8) = dijkstra(
            &0,
            |p| {
                buttons
                    .iter()
                    .map(|&b| (p.bitxor(b), 1))
                    .collect::<Vec<_>>()
            },
            |p| *p == target_lights,
        )
        .unwrap();
        #[cfg(test)]
        println!("{res:?}");
        total += res.1 as Output;

        while input[cursor] != b'}' {
            cursor += 1;
        }

        cursor += 2;
    }

    total
}

pub fn part1(puzzle: &str) -> Output {
    part_one(puzzle)
}

#[aoc(day10, part2)]
fn part_two(input: &str) -> Output {
    let input = input.as_bytes();
    let mut cursor = 0;
    let mut total = 0;

    while cursor < input.len() {
        let mut len = 0;
        while input[cursor] != b']' {
            len += 1;
            cursor += 1;
        }
        len -= 1;
        cursor += 2;

        let mut constraints_expressions = SmallVec::<[Expression; 10]>::with_capacity(len);
        for _ in 0..len {
            constraints_expressions.push(0.into());
        }
        let mut objective: Expression = 0.into();

        let mut vars = ProblemVariables::new();

        let mut buttons = Vec::new();
        while input[cursor] == b'(' {
            let mut button_nums = SmallVec::<[_; 10]>::new();
            let var = vars.add(variable().min(0).integer());
            objective += var;
            cursor += 1;
            while input[cursor] != b' ' {
                let (num, digits) = parse_initial_digits_unsigned_u32(&input[cursor..]);
                constraints_expressions[num as usize] += var;
                button_nums.push(num);
                cursor += digits + 1;
            }
            buttons.push(var);
            cursor += 1;
        }
        assert_eq!(input[cursor], b'{');
        cursor += 1;

        let mut problem = vars.minimise(objective.clone()).using(default_solver);

        let mut constraint_idx = 0;
        loop {
            let (num, digits) = parse_initial_digits_unsigned_u32(&input[cursor..]);
            problem.add_constraint(constraints_expressions[constraint_idx].clone().eq(num));
            cursor += digits;
            if input[cursor] == b'}' {
                break;
            }
            cursor += 1;
            constraint_idx += 1;
        }
        let res = problem.solve().expect("should be solvable");
        let cost = res.eval(objective);
        total += cost.round() as u64;

        cursor += 2;
    }

    total
}

pub fn part2(puzzle: &str) -> u64 {
    part_two(puzzle)
}

#[cfg(test)]
#[test]
fn example1() {
    let res = part_one(include_str!("test.txt"));
    assert_eq!(res, 7);
}

#[cfg(test)]
#[test]
fn example2() {
    let res = part2(include_str!("test.txt"));
    assert_eq!(res, 33);
}
