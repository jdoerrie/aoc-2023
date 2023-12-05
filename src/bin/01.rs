use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

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

fn rev_str(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = HashMap::from([
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

    let re = r"1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine";
    let start_re = Regex::new(re).unwrap();
    let rev_re = Regex::new(&rev_str(re)).unwrap();

    let mut total_sum = 0;
    for line in input.lines() {
        let first_digit = start_re.find(line).unwrap().as_str();
        let last_digit = rev_str(rev_re.find(&rev_str(line)).unwrap().as_str());

        let first_digit = numbers.get(first_digit).unwrap();
        let last_digit = numbers.get(last_digit.as_str()).unwrap();
        total_sum += first_digit * 10 + last_digit
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
