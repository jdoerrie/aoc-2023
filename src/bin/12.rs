advent_of_code::solution!(12);

use grid::Grid;
use rayon::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

use Spring::*;

fn parse_springs(line: &str) -> Vec<Spring> {
    line.chars()
        .map(|c| match c {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => panic!("Uknown spring"),
        })
        .collect()
}

fn parse_blocks(line: &str) -> Vec<usize> {
    line.split(',').flat_map(str::parse).collect()
}

type FastCache = Grid<Option<usize>>;
fn dp_fast_impl(springs: &[Spring], blocks: &[usize], cache: &mut FastCache) -> usize {
    if springs.is_empty() {
        return blocks.is_empty() as usize;
    }

    let opl = |springs: &[Spring], cache: &mut FastCache| {
        dp_fast_impl(
            &springs[springs
                .iter()
                .position(|&s| s != Operational)
                .unwrap_or(springs.len())..],
            blocks,
            cache,
        )
    };

    let dmg = |cache: &mut FastCache| {
        if blocks.is_empty() {
            0
        } else {
            let block0 = *blocks.first().unwrap();
            if block0 <= springs.len()
                && springs[0..block0].iter().all(|&s| s != Operational)
                && springs.get(block0).unwrap_or(&Operational) != &Damaged
            {
                dp_fast_impl(&springs[block0 + 1..], &blocks[1..], cache)
            } else {
                0
            }
        }
    };

    match springs[0] {
        Operational => opl(springs, cache),
        Damaged => dmg(cache),
        Unknown => {
            let cached = cache[(springs.len(), blocks.len())];
            if cached.is_none() {
                let v = opl(&springs[1..], cache) + dmg(cache);
                cache[(springs.len(), blocks.len())] = Some(v);
            }
            cache[(springs.len(), blocks.len())].unwrap()
        }
    }
}

fn dp_fast(springs: &[Spring], blocks: &[usize]) -> usize {
    let mut cache = Grid::new(springs.len() + 1, blocks.len() + 1);
    dp_fast_impl(springs, blocks, &mut cache)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .par_lines()
            .map(|l| {
                let (springs, blocks) = l.split_once(' ').unwrap();
                let springs = [springs; 1].join("?") + ".";
                let blocks = [blocks; 1].join(",");
                let springs = parse_springs(&springs);
                let blocks = parse_blocks(&blocks);
                dp_fast(&springs, &blocks)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .par_lines()
            .map(|l| {
                let (springs, blocks) = l.split_once(' ').unwrap();
                let springs = [springs; 5].join("?") + ".";
                let blocks = [blocks; 5].join(",");
                let springs = parse_springs(&springs);
                let blocks = parse_blocks(&blocks);
                dp_fast(&springs, &blocks)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
