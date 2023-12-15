use std::collections::{BTreeMap, HashMap, VecDeque};

use grid::{grid, Grid};

advent_of_code::solution!(14);

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Rock {
    Round,
    Cube,
}

enum Tilt {
    North,
    West,
    South,
    East,
}

use itertools::Itertools;
use Rock::*;

type Rocks = Grid<Option<Rock>>;
fn parse_rocks(input: &str) -> Rocks {
    let mut rocks = Rocks::new(input.lines().count(), input.lines().next().unwrap().len());
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.char_indices() {
            rocks[(i, j)] = match c {
                'O' => Some(Round),
                '#' => Some(Cube),
                '.' => None,
                _ => panic!("Unknown rock"),
            }
        }
    }

    rocks
}

fn do_west_shift(rocks: &mut Rocks) {
    for i in 0..rocks.size().0 {
        let mut row = rocks.iter_row(i).copied().collect_vec();
        let mut j = 0;
        for i in 0..row.len() {
            match row[i] {
                Some(Round) => {
                    row.swap(i, j);
                    j += 1;
                }
                Some(Cube) => {
                    j = i + 1;
                }
                None => {}
            }
        }

        for (x, y) in rocks.iter_row_mut(i).zip(row.iter()) {
            *x = *y;
        }
    }
}

fn do_shift(rocks: &[&Option<Rock>]) -> Vec<Option<Rock>> {
    let mut new_rocks = Vec::with_capacity(rocks.len());
    for (i, &rock) in rocks.iter().enumerate() {
        match rock {
            Some(Round) => new_rocks.push(Some(Round)),
            Some(Cube) => {
                while new_rocks.len() < i {
                    new_rocks.push(None);
                }
                new_rocks.push(Some(Cube));
            }
            None => {}
        }
    }
    while new_rocks.len() < rocks.len() {
        new_rocks.push(None);
    }
    new_rocks
}

fn do_cycle(mut rocks: Rocks) -> Rocks {
    rocks.rotate_left();
    for _ in 0..4 {
        do_west_shift(&mut rocks);
        rocks.rotate_right();
    }

    rocks.rotate_right();
    rocks
}

pub fn part_one(input: &str) -> Option<usize> {
    let rocks = parse_rocks(input);
    Some(
        rocks
            .iter_cols()
            .map(|col| {
                let mut free_idx = 0;
                let mut load = 0;
                for (i, r) in col.enumerate() {
                    match r {
                        Some(Round) => {
                            load += rocks.size().0 - free_idx;
                            free_idx += 1;
                        }
                        Some(Cube) => {
                            free_idx = i + 1;
                        }
                        None => {}
                    }
                }
                load
            })
            .sum(),
    )
}

fn calc_load(rocks: &Rocks) -> usize {
    rocks
        .iter_cols()
        .map(|col| {
            col.enumerate()
                .map(|(i, r)| (rocks.size().0 - i) * (*r == Some(Round)) as usize)
                .sum::<usize>()
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut rocks = parse_rocks(input);
    let mut lru_cache = VecDeque::new();
    let n_cycles = 1_000_000_000;
    let cache_size = 100000;
    lru_cache.push_back(rocks.clone());
    for i in 1..=n_cycles {
        rocks = do_cycle(rocks);
        if let Some(j) = lru_cache.iter().position(|r| *r == rocks) {
            println!("Breaking in loop {i}: {j}");
            let cycle_len = (i % cache_size) - j;
            let remaining = (n_cycles - i) % cycle_len;
            for _ in 0..remaining {
                rocks = do_cycle(rocks);
            }

            break;
        }

        lru_cache.push_back(rocks.clone());
        if lru_cache.len() > cache_size {
            lru_cache.pop_front();
        }
    }

    Some(calc_load(&rocks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
