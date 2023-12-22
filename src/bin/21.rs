use std::collections::{HashMap, VecDeque};

use arrayvec::ArrayVec;
use grid::{grid, Grid};

advent_of_code::solution!(21);

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
enum Tile {
    Start,
    Rock,
    Garden,
}

use itertools::Itertools;
use Tile::*;

type Tiles = Grid<Tile>;

fn next_moves(tiles: &Tiles, [r, c]: [usize; 2]) -> ArrayVec<[usize; 2], 4> {
    [
        [r.wrapping_sub(1), c],
        [r.wrapping_add(1), c],
        [r, c.wrapping_sub(1)],
        [r, c.wrapping_add(1)],
    ]
    .into_iter()
    .filter(|[r, c]| tiles.get(*r, *c).is_some_and(|&t| t != Rock))
    .collect()
}

fn next_moves_inf(tiles: &Tiles, [r, c]: [isize; 2]) -> ArrayVec<[isize; 2], 4> {
    [[r - 1, c], [r + 1, c], [r, c - 1], [r, c + 1]]
        .into_iter()
        .filter(|[r, c]| {
            tiles
                .get(
                    ((*r).rem_euclid(tiles.rows() as isize)) as usize,
                    ((*c).rem_euclid(tiles.cols() as isize)) as usize,
                )
                .is_some_and(|&t| t != Rock)
        })
        .collect()
}

fn parse_tiles(input: &str) -> Tiles {
    let mut tiles = grid![];

    for l in input.lines() {
        tiles.push_row(
            l.chars()
                .map(|c| match c {
                    'S' => Start,
                    '.' => Garden,
                    '#' => Rock,
                    _ => unreachable!("Unknown Tile"),
                })
                .collect(),
        );
    }

    tiles
}

fn get_distances(tiles: &Tiles) -> Grid<Option<usize>> {
    let (start, _) = tiles.indexed_iter().find(|(_, t)| **t == Start).unwrap();

    let mut distances = Grid::new(tiles.rows(), tiles.cols());
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    distances[start] = Some(0);

    while let Some(((r, c), d)) = q.pop_front() {
        q.extend(next_moves(tiles, [r, c]).iter().filter_map(|&[r, c]| {
            if distances[(r, c)].is_none() {
                distances[(r, c)] = Some(d + 1);
                Some(((r, c), d + 1))
            } else {
                None
            }
        }));
    }

    distances
}

fn get_distances_inf(tiles: &Tiles, max_dist: usize) -> HashMap<[isize; 2], usize> {
    let (start, _) = tiles.indexed_iter().find(|(_, t)| **t == Start).unwrap();
    let start = [start.0 as isize, start.1 as isize];

    let mut distances = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    distances.insert(start, 0);

    while let Some((pos, d)) = q.pop_front() {
        q.extend(next_moves_inf(tiles, pos).iter().filter_map(|&pos| {
            if !distances.contains_key(&pos) && d < max_dist {
                distances.insert(pos, d + 1);
                Some((pos, d + 1))
            } else {
                None
            }
        }));
    }

    distances
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        get_distances(&parse_tiles(input))
            .iter()
            .filter(|d| d.is_some_and(|d| d % 2 == 0 && d <= 64))
            .count(),
    )
}

fn eval_quad(xs: &[usize; 3], ys: &[usize; 3], x: usize) -> usize {
    let x0 = xs[0];
    let x1 = xs[1];
    let x2 = xs[2];

    let y0 = ys[0];
    let y1 = ys[1];
    let y2 = ys[2];

    y0 + (y1 - y0) * (x - x0) / (x1 - x0)
        + (x - x0)
            * (x - x1)
            * ((y2 - y1) / ((x2 - x1) * (x2 - x0)) - (y1 - y0) / ((x1 - x0) * (x2 - x0)))
}

pub fn part_two(input: &str) -> Option<usize> {
    let n = 26501365;
    let tiles = parse_tiles(input);
    assert_eq!(tiles.rows(), tiles.cols());
    let m = tiles.rows();

    let dists = get_distances_inf(&tiles, 131 * 2 + 65);
    let xs = [0, 1, 2];
    let ys: [usize; 3] = xs
        .iter()
        .map(|i| (n % m) + i * m)
        .map(|x| {
            dists
                .values()
                .filter(|&&v| v % 2 == x % 2 && v <= x)
                .count()
        })
        .collect_vec()
        .try_into()
        .unwrap();
    Some(eval_quad(&xs, &ys, n / m))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(394693535848011));
    }
}
