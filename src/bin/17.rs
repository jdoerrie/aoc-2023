use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use arrayvec::ArrayVec;
use grid::{grid, Grid};

advent_of_code::solution!(17);

type Heats = Grid<usize>;

fn parse(input: &str) -> Heats {
    let mut heats = grid![];
    for line in input.lines() {
        heats.push_row(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
        )
    }
    heats
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

use Dir::*;

type Pos = (usize, usize);

type State = (Pos, Dir, usize);

fn next_pos((m, n): (usize, usize), (r, c): Pos, dir: Dir) -> Option<Pos> {
    let (r, c) = match dir {
        Up => (r.wrapping_sub(1), c),
        Down => (r.wrapping_add(1), c),
        Left => (r, c.wrapping_sub(1)),
        Right => (r, c.wrapping_add(1)),
    };

    if (0..m).contains(&r) && (0..n).contains(&c) {
        Some((r, c))
    } else {
        None
    }
}

fn next(
    dims: (usize, usize),
    (pos, dir, cnt): State,
    min_cnt: usize,
    max_cnt: usize,
) -> ArrayVec<State, 3> {
    [Up, Down, Left, Right]
        .into_iter()
        .filter(|&d| match (d, dir) {
            (Up, Down) => false,
            (Down, Up) => false,
            (Left, Right) => false,
            (Right, Left) => false,
            (x, y) => (x != y && cnt >= min_cnt) || (x == y && cnt < max_cnt),
        })
        .filter(|&d| next_pos(dims, pos, d).is_some())
        .map(|d| {
            (
                next_pos(dims, pos, d).unwrap(),
                d,
                ((d == dir) as usize * cnt) + 1,
            )
        })
        .collect()
}

fn solve(heats: &Heats, min_cnt: usize, max_cnt: usize) -> usize {
    let mut costs = HashMap::new();
    let mut pq = BinaryHeap::new();

    let goal = (heats.size().0 - 1, heats.size().1 - 1);
    let down: State = ((0, 0), Down, 0);
    let right: State = ((0, 0), Right, 0);
    let cost = 0;
    costs.insert(down, cost);
    costs.insert(right, cost);
    pq.push((Reverse(cost), down));
    pq.push((Reverse(cost), right));
    while let Some((Reverse(cost), state)) = pq.pop() {
        if state.0 == goal && state.2 >= min_cnt {
            return cost;
        }

        for next in next(heats.size(), state, min_cnt, max_cnt) {
            let next_cost = cost + heats[next.0];
            let entry = costs.entry(next).or_insert(usize::MAX);
            if next_cost < *entry {
                *entry = next_cost;
                pq.push((Reverse(next_cost), next));
            }
        }
    }

    unreachable!("No Goal");
}

pub fn part_one(input: &str) -> Option<usize> {
    let heats = parse(input);
    Some(solve(&heats, 0, 3))
}

pub fn part_two(input: &str) -> Option<usize> {
    let heats = parse(input);
    Some(solve(&heats, 4, 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
