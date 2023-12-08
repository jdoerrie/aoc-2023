use std::collections::HashMap;

use itertools::Itertools;
use num::Integer;
use tuple::Map;

advent_of_code::solution!(8);

enum Direction {
    Left,
    Right,
}

fn parse_dirs(line: &str) -> Vec<Direction> {
    line.chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown Direction"),
        })
        .collect()
}

fn parse_node(node: &str) -> i64 {
    i64::from_str_radix(node, 36).unwrap()
}

fn is_start(node: i64) -> bool {
    node % 36 == 10
}

fn is_end(node: i64) -> bool {
    node % 36 == 35
}

type Network = HashMap<i64, (i64, i64)>;

fn parse_network(lines: &str) -> Network {
    HashMap::from_iter(lines.lines().map(|line| {
        let (key, val) = line.split_once(" = ").unwrap();
        let (lhs, rhs) = val[1..9].split_once(", ").unwrap();
        (parse_node(key), (parse_node(lhs), parse_node(rhs)))
    }))
}

fn traverse_network(net: &Network, dirs: &[Direction]) -> usize {
    let (start, end) = ("AAA", "ZZZ").map(parse_node);
    let mut node = start;
    for (i, dir) in dirs.iter().cycle().enumerate() {
        node = match dir {
            Direction::Left => net.get(&node).unwrap().0,
            Direction::Right => net.get(&node).unwrap().1,
        };
        if node == end {
            return i + 1;
        }
    }

    panic!("No Dirs")
}

fn traverse_network_ghost(net: &Network, dirs: &[Direction]) -> usize {
    let mut nodes = net.keys().copied().filter(|&k| is_start(k)).collect_vec();

    let mut n_cycle = 0;
    let mut cycle_lens = vec![None; nodes.len()];
    while cycle_lens.iter().any(|l| l.is_none()) {
        n_cycle += 1;
        for dir in dirs {
            nodes = nodes
                .iter()
                .map(|node| match dir {
                    Direction::Left => net.get(node).unwrap().0,
                    Direction::Right => net.get(node).unwrap().1,
                })
                .collect();
        }

        for (i, cycles) in cycle_lens.iter_mut().enumerate() {
            if cycles.is_none() && is_end(nodes[i]) {
                *cycles = Some(n_cycle);
            }
        }
    }

    cycle_lens
        .iter()
        .map(|c| c.unwrap())
        .reduce(|acc, c| acc.lcm(&c))
        .unwrap()
        * dirs.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (dirs, net) = input.split_once("\n\n").unwrap();
    let dirs = parse_dirs(dirs);
    let net = parse_network(net);
    Some(traverse_network(&net, &dirs))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (dirs, net) = input.split_once("\n\n").unwrap();
    let dirs = parse_dirs(dirs);
    let net = parse_network(net);
    Some(traverse_network_ghost(&net, &dirs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
