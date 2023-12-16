advent_of_code::solution!(16);

use rayon::prelude::*;

enum Tile {
    MirrorLF,
    Mirror7L,
    SplitterV,
    SplitterH,
}

use Tile::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

use grid::{grid, Grid};
use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use Dir::*;

type Tiles = Grid<Option<Tile>>;

fn parse(input: &str) -> Grid<Option<Tile>> {
    let mut grid = grid![];
    for l in input.lines() {
        grid.push_row(
            l.chars()
                .map(|c| match c {
                    '.' => None,
                    '|' => Some(SplitterV),
                    '-' => Some(SplitterH),
                    '\\' => Some(Mirror7L),
                    '/' => Some(MirrorLF),
                    _ => panic!(""),
                })
                .collect_vec(),
        );
    }

    grid
}

type Pos = (usize, usize);

fn move_pos((n, m): Pos, (r, c): Pos, dir: Dir) -> Option<(Pos, Dir)> {
    let (r, c) = match dir {
        Up => (r.wrapping_sub(1), c),
        Down => (r.wrapping_add(1), c),
        Left => (r, c.wrapping_sub(1)),
        Right => (r, c.wrapping_add(1)),
    };

    if (0..n).contains(&r) && (0..m).contains(&c) {
        Some(((r, c), dir))
    } else {
        None
    }
}

type PosDir = (Pos, Dir);
fn do_step(grid: &Tiles, (r, c): Pos, dir: Dir) -> [Option<PosDir>; 2] {
    let mv = |dir| move_pos(grid.size(), (r, c), dir);
    match grid[(r, c)] {
        None => [mv(dir), None],
        Some(SplitterV) => match dir {
            Up | Down => [mv(dir), None],
            Left | Right => [mv(Up), mv(Down)],
        },
        Some(SplitterH) => match dir {
            Left | Right => [mv(dir), None],
            Up | Down => [mv(Left), mv(Right)],
        },
        Some(Mirror7L) => match dir {
            Up => [mv(Left), None],
            Down => [mv(Right), None],
            Left => [mv(Up), None],
            Right => [mv(Down), None],
        },
        Some(MirrorLF) => match dir {
            Up => [mv(Right), None],
            Down => [mv(Left), None],
            Left => [mv(Down), None],
            Right => [mv(Up), None],
        },
    }
}

fn solve(grid: &Tiles, pos: Pos, dir: Dir) -> usize {
    let mut seen = Grid::<u8>::new(grid.size().0, grid.size().1);
    let mut stack = Vec::new();
    let bit = |dir| match dir {
        Up => 1 << 0,
        Down => 1 << 1,
        Left => 1 << 2,
        Right => 1 << 3,
    };
    seen[pos] |= bit(dir);
    stack.push((pos, dir));
    while let Some((pos, dir)) = stack.pop() {
        for (pos, dir) in do_step(grid, pos, dir).into_iter().flatten() {
            if (seen[pos] & bit(dir)) == 0 {
                seen[pos] |= bit(dir);
                stack.push((pos, dir));
            }
        }
    }

    seen.iter().filter(|&x| *x != 0).count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    Some(solve(&grid, (0, 0), Right))
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    let (m, n) = grid.size();
    let mut starts = Vec::with_capacity(2 * (m + n));
    for r in 0..m {
        starts.push(((r, 0), Right));
        starts.push(((r, n - 1), Left));
    }
    for c in 0..n {
        starts.push(((0, c), Down));
        starts.push(((m - 1, c), Up));
    }

    starts
        .par_iter()
        .map(|&(pos, dir)| solve(&grid, pos, dir))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
