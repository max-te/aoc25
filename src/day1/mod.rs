use aoc_runner_derive::{aoc, aoc_generator};

type Output = u64;

#[aoc(day1, part1)]
fn part_one(input: &str) -> Output {
    let mut position = 50;
    let mut zeroes = 0;

    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let amount = line[1..].parse::<i16>().unwrap();
        position = (position
            + match direction {
                'L' => -amount,
                'R' => amount,
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
    let mut position = 50;
    let mut zeroes = 0;

    for line in input.lines() {
        let started_at_zero = position == 0;
        let direction = line.chars().next().unwrap();
        let mut amount = line[1..].parse::<i16>().unwrap();
        if amount >= 100 {
            zeroes += (amount / 100) as u64;
            amount %= 100;
        }

        position += match direction {
            'L' => -amount,
            'R' => amount,
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
