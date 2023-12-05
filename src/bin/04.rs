use std::collections::HashSet;

use tuple::Map;

advent_of_code::solution!(4);

fn split_into_set(input: &str) -> HashSet<u32> {
    input
        .split_ascii_whitespace()
        .flat_map(|n| n.parse())
        .collect()
}

fn parse_line(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    line.split_once(": ")
        .unwrap()
        .1
        .split_once(" | ")
        .unwrap()
        .map(split_into_set)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (winning_numbers, scratch_card) = parse_line(line);
                match winning_numbers.intersection(&scratch_card).count() {
                    0 => 0,
                    i => 1 << (i - 1),
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut counts = vec![1; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        let (winning_numbers, scratch_card) = parse_line(line);
        let matching_nums = winning_numbers.intersection(&scratch_card).count();
        for j in i + 1..=i + matching_nums {
            counts[j] += counts[i];
        }
    }
    Some(counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
