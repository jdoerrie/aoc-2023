use std::collections::HashMap;

use grid::Grid;

advent_of_code::solution!(14);

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Rock {
    Round,
    Cube,
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

fn do_cycle(rocks: &mut Rocks) {
    rocks.rotate_left();
    for _ in 0..4 {
        do_west_shift(rocks);
        rocks.rotate_right();
    }

    rocks.rotate_right();
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
    let n_cycles = 1_000_000_000;
    let mut cache = HashMap::new();
    let mut loads = Vec::new();
    let load = calc_load(&rocks);
    loads.push(load);
    cache.insert(load, vec![(0, rocks.clone())]);
    for i in 1..=n_cycles {
        do_cycle(&mut rocks);

        let load = calc_load(&rocks);
        loads.push(load);
        let v = cache.entry(load).or_default();
        if let Some(j) = v.iter().position(|r| r.1 == rocks) {
            let j = v[j].0;
            let cycle_len = i - j;
            let remaining = (n_cycles - i) % cycle_len;
            return Some(loads[remaining + j]);
        }

        v.push((i, rocks.clone()));
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
