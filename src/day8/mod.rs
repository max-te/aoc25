use rustc_hash::{FxBuildHasher, FxHashSet};

use aoc_runner_derive::aoc;
use petgraph::unionfind::UnionFind;

use crate::util::parse_initial_digits;

type Output = usize;

fn dist(node_a: (i64, i64, i64), node_b: (i64, i64, i64)) -> i64 {
    (node_a.0 - node_b.0).pow(2) + (node_a.1 - node_b.1).pow(2) + (node_a.2 - node_b.2).pow(2)
}

fn part_one_inner(input: &str, connections_count: usize) -> Output {
    let input = input.as_bytes();

    // Parse
    let mut positions = Vec::with_capacity(1000);
    let mut cursor = 0;
    while cursor < input.len() {
        let (x, xdigits) = parse_initial_digits(&input[cursor..]);
        cursor += xdigits + 1;
        let (y, ydigits) = parse_initial_digits(&input[cursor..]);
        cursor += ydigits + 1;
        let (z, zdigits) = parse_initial_digits(&input[cursor..]);
        cursor += zdigits + 1;
        positions.push((x, y, z));
    }

    // Find shortest connections
    let mut shortest = Vec::<(i64, usize, usize)>::with_capacity(connections_count);
    for (idx_a, node_a) in positions.iter().enumerate() {
        for (idx_b, node_b) in positions.iter().enumerate().skip(idx_a + 1) {
            let sq_dist = dist(*node_a, *node_b);
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
    let mut connected_nodes =
        FxHashSet::with_capacity_and_hasher(positions.len(), FxBuildHasher::default());
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
        let (x, xdigits) = parse_initial_digits(&input[cursor..]);
        cursor += xdigits + 1;
        let (y, ydigits) = parse_initial_digits(&input[cursor..]);
        cursor += ydigits + 1;
        let (z, zdigits) = parse_initial_digits(&input[cursor..]);
        cursor += zdigits + 1;
        positions.push((x, y, z));
    }

    let mut longest_in_tracked = (0, 0, 0);

    let mut shortest_to_tracked = Vec::with_capacity(positions.len());
    for (idx, p) in positions.iter().enumerate() {
        let sq_dist = dist(positions[0], *p);
        shortest_to_tracked.push((sq_dist, 0, idx));
    }

    let mut tracked_component =
        FxHashSet::with_capacity_and_hasher(positions.len(), FxBuildHasher::default());
    tracked_component.insert(0);

    while tracked_component.len() < positions.len() {
        let new_connection = shortest_to_tracked
            .iter()
            .filter(|x| x.0 > 0)
            .min_by_key(|x| x.0)
            .unwrap()
            .clone();
        tracked_component.insert(new_connection.2);
        shortest_to_tracked[new_connection.2] = (0, new_connection.2, new_connection.2);
        let new_pos = positions[new_connection.2];

        for (unconn_idx, unconn_node) in positions.iter().enumerate() {
            if tracked_component.contains(&unconn_idx) {
                continue;
            }
            let sq_dist = dist(*unconn_node, new_pos);
            if sq_dist < shortest_to_tracked[unconn_idx].0 {
                shortest_to_tracked[unconn_idx] = (sq_dist, new_connection.2, unconn_idx);
            }
        }
        if new_connection.0 > longest_in_tracked.0 {
            longest_in_tracked = new_connection;
        }
    }

    (positions[longest_in_tracked.1].0 * positions[longest_in_tracked.2].0) as u64
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
