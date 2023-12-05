use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect_vec())
            .map(|v| v.first().unwrap_or(&0) * 10 + v.last().unwrap_or(&0))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut total_sum = 0;
    for line in input.lines() {
        let first_digit = numbers
            .iter()
            .filter_map(|(k, v)| (line.find(k).map(|i| (i, v))))
            .min();
        let last_digit = numbers
            .iter()
            .filter_map(|(k, v)| (line.rfind(k).map(|i| (i, v))))
            .max();
        total_sum += first_digit.unwrap().1 * 10 + last_digit.unwrap().1;
    }

    Some(total_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
