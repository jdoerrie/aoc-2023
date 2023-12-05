use std::ops::Range;

use itertools::Itertools;
use rangemap::RangeMap;

advent_of_code::solution!(5);

fn parse_nums(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .flat_map(str::parse)
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<i64> {
    let seeds = parse_nums(input.lines().next().unwrap().split_once(": ").unwrap().1);

    let maps = input
        .lines()
        .skip(1)
        .group_by(|l| l.is_empty())
        .into_iter()
        .filter_map(|(is_empty, group)| if is_empty { None } else { Some(group) })
        .map(|group| {
            let mut map = RangeMap::new();
            map.extend(group.skip(1).map(|line| {
                let (dst, src, len) = parse_nums(line).into_iter().collect_tuple().unwrap();
                (
                    Range {
                        start: src,
                        end: src + len,
                    },
                    dst - src,
                )
            }));
            map
        })
        .collect_vec();

    Some(
        *(maps
            .iter()
            .fold(seeds.clone(), |acc, map| {
                acc.into_iter()
                    .map(|i| i + map.get(&i).unwrap_or(&0))
                    .collect_vec()
            })
            .iter()
            .min()
            .unwrap()),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
