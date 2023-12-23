use std::collections::{HashSet, VecDeque};

use arrayvec::ArrayVec;
use itertools::Itertools;
use ndarray::Array2;

advent_of_code::solution!(23);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
    Path,
    Forest,
    Slope(Dir),
}

use Dir::*;
use Tile::*;

type Grid = Array2<Tile>;

fn parse(input: &str) -> Grid {
    let shape = (input.lines().count(), input.lines().next().unwrap().len());
    let v = input
        .lines()
        .flat_map(|l| {
            l.chars().map(|c| match c {
                '.' => Path,
                '#' => Forest,
                '<' => Slope(Left),
                '^' => Slope(Up),
                'v' => Slope(Down),
                '>' => Slope(Right),
                _ => unreachable!("Unknown Tile"),
            })
        })
        .collect_vec();
    Grid::from_shape_vec(shape, v).unwrap()
}

fn parse2(input: &str) -> Grid {
    let shape = (input.lines().count(), input.lines().next().unwrap().len());
    let v = input
        .lines()
        .flat_map(|l| {
            l.chars().map(|c| match c {
                '.' | '<' | 'v' | '>' | '^' => Path,
                '#' => Forest,
                _ => unreachable!("Unknown Tile"),
            })
        })
        .collect_vec();
    Grid::from_shape_vec(shape, v).unwrap()
}

fn pois(grid: &Grid) -> Vec<[usize; 2]> {
    grid.indexed_iter()
        .filter(|&(_, t)| *t != Forest)
        .filter(|&((r, c), _)| {
            r == 0
                || r + 1 == grid.shape()[0]
                || [
                    [r.wrapping_sub(1), c],
                    [r.wrapping_add(1), c],
                    [r, c.wrapping_sub(1)],
                    [r, c.wrapping_add(1)],
                ]
                .iter()
                .filter(|&pos| grid.get(*pos).is_some_and(|&t| t != Forest))
                .count()
                    > 2
        })
        .map(|((r, c), _)| [r, c])
        .collect()
}

fn find_neighbors(grid: &Grid, u: [usize; 2], pois: &[[usize; 2]]) -> Vec<([usize; 2], usize)> {
    let mut seen = HashSet::new();
    let mut res = vec![];
    let mut q = VecDeque::new();
    q.push_back((u, 0));
    seen.insert(u);
    while let Some((v, d)) = q.pop_front() {
        if v != u && pois.iter().any(|&p| p == v) {
            res.push((v, d));
        } else {
            let [r, c] = v;
            q.extend(
                [
                    [r.wrapping_sub(1), c],
                    [r.wrapping_add(1), c],
                    [r, c.wrapping_sub(1)],
                    [r, c.wrapping_add(1)],
                ]
                .into_iter()
                .filter(|&pos| grid.get(pos).is_some_and(|&t| t != Forest) && seen.insert(pos))
                .map(|pos| (pos, d + 1)),
            );
        }
    }

    res
}

fn dfs_impl(grid: &Grid, mut path: Vec<[usize; 2]>) -> Option<usize> {
    let &[r, c] = path.last().unwrap();
    if r + 1 == grid.shape()[0] && grid[[r, c]] == Path {
        // println!("Found Path of Len {}", path.len() - 1);
        return Some(path.len() - 1);
    }

    let mut next = ArrayVec::<[usize; 2], 4>::new();
    match grid[[r, c]] {
        Path => next.extend([
            [r.wrapping_sub(1), c],
            [r.wrapping_add(1), c],
            [r, c.wrapping_sub(1)],
            [r, c.wrapping_add(1)],
        ]),

        Slope(Up) => next.push([r.wrapping_sub(1), c]),
        Slope(Down) => next.push([r.wrapping_add(1), c]),
        Slope(Left) => next.push([r, c.wrapping_sub(1)]),
        Slope(Right) => next.push([r, c.wrapping_add(1)]),
        Forest => panic!("Unexpected Forest"),
    };

    let filtered = ArrayVec::<[usize; 2], 4>::from_iter(
        next.into_iter()
            .filter(|n| grid.get(*n).is_some_and(|t| *t != Forest) && path.iter().all(|p| p != n)),
    );

    match filtered.len() {
        0 => None,
        1 => {
            path.push(filtered[0]);
            dfs_impl(grid, path)
        }
        _ => filtered.into_iter().fold(None, |acc, e| {
            let mut p = path.clone();
            p.push(e);
            let res = dfs_impl(grid, p);
            if res.is_none() {
                acc
            } else {
                res.map(|l| l.max(acc.unwrap_or(usize::MIN)))
            }
        }),
    }
}

fn dfs(grid: &Grid) -> Option<usize> {
    let start = [0, grid.row(0).iter().position(|&t| t == Path).unwrap()];
    dfs_impl(grid, vec![start])
}

type Graph = Vec<ArrayVec<(usize, usize), 4>>;

fn dfs_compressed_impl(g: &Graph, u: usize, seen: u64, end: usize) -> Option<usize> {
    if u == end {
        return Some(0);
    }

    let mut cost = None;
    for (v, d) in g[u].iter() {
        if seen & (1 << v) == 0 {
            let res = dfs_compressed_impl(g, *v, seen | (1 << v), end);
            if res.is_some() {
                cost = Some(cost.unwrap_or(0).max(d + res.unwrap()));
            }
        }
    }

    cost
}

fn dfs_compressed(g: &Graph, start: usize, end: usize) -> Option<usize> {
    dfs_compressed_impl(g, start, 1 << start, end)
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    dfs(&grid)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse2(input);
    let pois = pois(&grid);
    let compressed: Graph = pois
        .iter()
        .map(|p| {
            ArrayVec::<(usize, usize), 4>::from_iter(
                find_neighbors(&grid, *p, &pois)
                    .into_iter()
                    .map(|(v, d)| (pois.iter().position(|p| *p == v).unwrap(), d)),
            )
        })
        .collect();
    dfs_compressed(&compressed, 0, pois.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
