use std::collections::HashSet;

use aoc_runner_derive::aoc;
use petgraph::unionfind::UnionFind;

use crate::util::parse_initial_digits_unsigned_u64;

type Output = usize;

fn part_one_inner(input: &str, connections_count: usize) -> Output {
    let input = input.as_bytes();

    // Parse
    let mut positions = Vec::with_capacity(1000);
    let mut cursor = 0;
    while cursor < input.len() {
        let (x, xdigits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += xdigits + 1;
        let (y, ydigits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += ydigits + 1;
        let (z, zdigits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += zdigits + 1;
        positions.push((x, y, z));
    }

    // Find shortest connections
    let mut shortest = Vec::<(u64, usize, usize)>::with_capacity(connections_count);
    for (idx_a, node_a) in positions.iter().enumerate() {
        for (idx_b, node_b) in positions.iter().enumerate().skip(idx_a + 1) {
            let sq_dist = (node_a.0.abs_diff(node_b.0)).pow(2)
                + (node_a.1.abs_diff(node_b.1)).pow(2)
                + (node_a.2.abs_diff(node_b.2)).pow(2);
            if shortest.len() < connections_count {
                shortest.insert(
                    shortest.partition_point(|&x| x.0 < sq_dist),
                    (sq_dist, idx_a, idx_b),
                );
            } else if shortest.last().unwrap().0 > sq_dist {
                shortest.pop();

                shortest.insert(
                    shortest.partition_point(|&x| x.0 < sq_dist),
                    (sq_dist, idx_a, idx_b),
                );
            }
        }
    }

    // Determine components
    let mut connected_nodes = HashSet::new();
    let mut network = UnionFind::<usize>::new(positions.len());
    for connection in shortest {
        network.union(connection.1, connection.2);
        connected_nodes.insert(connection.1);
        connected_nodes.insert(connection.2);
    }
    // Find 3 largest
    let mut comp_sizes = Vec::with_capacity(connected_nodes.len());
    for node in connected_nodes.iter() {
        if network.find(*node) == *node {
            let comp_size = connected_nodes
                .iter()
                .filter(|&other| network.find(*other) == *node)
                .count();
            comp_sizes.push(comp_size);
        }
    }
    comp_sizes.sort_unstable();
    let mut product = 1;
    for _ in 0..3 {
        product *= comp_sizes.pop().unwrap();
    }

    product
}

#[aoc(day8, part1)]
fn part_one(input: &str) -> Output {
    part_one_inner(input, 1000)
}

pub fn part1(puzzle: &str) -> Output {
    part_one(puzzle)
}

#[aoc(day8, part2)]
fn part_two(input: &str) -> u64 {
    let input = input.as_bytes();

    // Parse
    let mut positions = Vec::with_capacity(1000);
    let mut cursor = 0;
    while cursor < input.len() {
        let (x, xdigits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += xdigits + 1;
        let (y, ydigits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += ydigits + 1;
        let (z, zdigits) = parse_initial_digits_unsigned_u64(&input[cursor..]);
        cursor += zdigits + 1;
        positions.push((x, y, z));
    }

    let mut network = UnionFind::<usize>::new(positions.len());

    // Find shortest connections
    let mut edges = Vec::<(u64, usize, usize)>::with_capacity(positions.len().pow(2));
    for (idx_a, node_a) in positions.iter().enumerate() {
        for (idx_b, node_b) in positions.iter().enumerate().skip(idx_a + 1) {
            let sq_dist = (node_a.0.abs_diff(node_b.0)).pow(2)
                + (node_a.1.abs_diff(node_b.1)).pow(2)
                + (node_a.2.abs_diff(node_b.2)).pow(2);
            edges.push((sq_dist, idx_a, idx_b));
        }
    }
    edges.sort_by_key(|x| x.0);

    let mut last_connection = (0, 0);
    for connection in edges {
        last_connection = (connection.1, connection.2);

        if network.union(connection.1, connection.2) {
            let root = network.find(0);
            if (0..positions.len()).all(|p| network.find(p) == root) {
                break;
            }
        }
    }

    positions[last_connection.0].0 * positions[last_connection.1].0
}

pub fn part2(puzzle: &str) -> u64 {
    part_two(puzzle)
}

#[cfg(test)]
#[test]
fn example1() {
    let res = part_one_inner(include_str!("test.txt"), 10);
    assert_eq!(res, 40);
}

#[cfg(test)]
#[test]
fn example2() {
    let res = part2(include_str!("test.txt"));
    assert_eq!(res, 25272);
}
