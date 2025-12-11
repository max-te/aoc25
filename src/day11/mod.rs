use aoc_runner_derive::aoc;
use rustc_hash::FxHashMap;

type Output = usize;

#[aoc(day11, part1)]
fn part_one(input: &str) -> Output {
    let input = input.as_bytes();
    let mut cursor = 0;

    let mut connections = FxHashMap::default();

    while cursor < input.len() {
        let input_name = [input[cursor], input[cursor + 1], input[cursor + 2]];
        cursor += 4;

        let mut outputs = Vec::new();
        while cursor < input.len() && input[cursor] == b' ' {
            let output_name = [input[cursor + 1], input[cursor + 2], input[cursor + 3]];
            outputs.push(output_name);
            cursor += 4;
        }
        connections.insert(input_name, outputs);
        cursor += 1;
    }

    pathfinding::directed::count_paths::count_paths(
        b"you",
        |input| connections.get(*input).unwrap(),
        |node| *node == b"out",
    )
}

pub fn part1(puzzle: &str) -> Output {
    part_one(puzzle)
}

#[aoc(day11, part2)]
fn part_two(input: &str) -> Output {
    let input = input.as_bytes();
    let mut cursor = 0;

    let mut connections = FxHashMap::default();

    while cursor < input.len() {
        let input_name = [input[cursor], input[cursor + 1], input[cursor + 2]];
        cursor += 4;

        let mut outputs = Vec::new();
        while cursor < input.len() && input[cursor] == b' ' {
            let output_name = [input[cursor + 1], input[cursor + 2], input[cursor + 3]];
            outputs.push(output_name);
            cursor += 4;
        }
        connections.insert(input_name, outputs);
        cursor += 1;
    }
    connections.insert(*b"out", Vec::new());

    let count_paths = |from: &[u8; 3], to: &[u8; 3]| {
        pathfinding::directed::count_paths::count_paths(
            from,
            |input| connections.get(*input).unwrap(),
            |node| *node == to,
        )
    };

    count_paths(b"svr", b"fft") * count_paths(b"fft", b"dac") * count_paths(b"dac", b"out")
}

pub fn part2(puzzle: &str) -> Output {
    part_two(puzzle)
}

#[cfg(test)]
#[test]
fn example1() {
    let res = part_one(include_str!("test.txt"));
    assert_eq!(res, 5);
}

#[cfg(test)]
#[test]
fn example2() {
    let res = part2(include_str!("test2.txt"));
    assert_eq!(res, 2);
}
