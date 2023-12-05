use std::ops::Range;

use itertools::Itertools;
use rangemap::RangeMap;

advent_of_code::solution!(5);

fn parse_nums(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .flat_map(str::parse)
        .collect_vec()
}

fn parse_seeds(line: &str) -> Vec<i64> {
    parse_nums(line.split_once(": ").unwrap().1)
}

fn parse_seed_ranges(line: &str) -> Vec<Range<i64>> {
    parse_nums(line.split_once(": ").unwrap().1)
        .into_iter()
        .chunks(2)
        .into_iter()
        .map(|c| {
            let (start, size) = c.collect_tuple().unwrap();
            Range {
                start,
                end: start + size,
            }
        })
        .collect_vec()
}

fn parse_maps(input: &str) -> Vec<RangeMap<i64, i64>> {
    input
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
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<i64> {
    let seeds = parse_seeds(input.lines().next().unwrap());
    let maps = parse_maps(input);

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

pub fn part_two(input: &str) -> Option<i64> {
    let seeds = parse_seed_ranges(input.lines().next().unwrap());
    let maps = parse_maps(input);

    maps.iter()
        .fold(seeds.clone(), |acc, map| {
            acc.into_iter()
                .flat_map(|seeds| {
                    map.gaps(&seeds)
                        .chain(map.overlapping(&seeds).map(|(overlap, &offset)| Range {
                            start: seeds.start.max(overlap.start) + offset,
                            end: seeds.end.min(overlap.end) + offset,
                        }))
                        .collect_vec()
                })
                .collect_vec()
        })
        .iter()
        .map(|rng| rng.start)
        .min()
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
        assert_eq!(result, Some(46));
    }
}
