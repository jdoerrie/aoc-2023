advent_of_code::solution!(12);

#[derive(Clone, Copy, PartialEq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

use std::collections::HashMap;

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

type Cache = HashMap<(usize, usize, Option<usize>), usize>;
fn dp(springs: &[Spring], blocks: &[usize], curr_block: Option<usize>, cache: &mut Cache) -> usize {
    if springs.is_empty() {
        return if blocks.is_empty() && curr_block.is_none() {
            1
        } else {
            0
        };
    }

    let block0 = *blocks.first().unwrap_or(&0);
    if curr_block.unwrap_or(0) > block0 {
        return 0;
    }

    let damaged = |cache: &mut Cache| {
        dp(
            &springs[1..],
            blocks,
            Some(curr_block.unwrap_or(0) + 1),
            cache,
        )
    };
    let operational = |cache: &mut Cache| {
        dp(
            &springs[1..],
            if curr_block.is_none() {
                blocks
            } else {
                assert!(block0 == curr_block.unwrap());
                &blocks[1..]
            },
            None,
            cache,
        )
    };

    let key = (springs.len(), blocks.len(), curr_block);
    if let Some(val) = cache.get(&key) {
        return *val;
    }

    let val = match springs[0] {
        Damaged => damaged(cache),
        Operational => {
            if curr_block.is_some() && block0 != curr_block.unwrap() {
                0
            } else {
                operational(cache)
            }
        }
        Unknown => {
            if curr_block.unwrap_or(0) == block0 {
                operational(cache)
            } else {
                match curr_block {
                    None => operational(cache) + damaged(cache),
                    Some(_) => damaged(cache),
                }
            }
        }
    };

    cache.insert(key, val);
    val
}

fn dp_entry(springs: &[Spring], blocks: &[usize]) -> usize {
    let mut cache = HashMap::new();
    dp(springs, blocks, None, &mut cache)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|l| {
                let (springs, blocks) = l.split_once(' ').unwrap();
                let springs = [springs; 1].join("?") + ".";
                let blocks = [blocks; 1].join(",");
                let springs = parse_springs(&springs);
                let blocks = parse_blocks(&blocks);
                dp_entry(&springs, &blocks)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|l| {
                let (springs, blocks) = l.split_once(' ').unwrap();
                let springs = [springs; 5].join("?") + ".";
                let blocks = [blocks; 5].join(",");
                let springs = parse_springs(&springs);
                let blocks = parse_blocks(&blocks);
                dp_entry(&springs, &blocks)
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
