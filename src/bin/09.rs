use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Copy, Clone)]
enum Part {
    One,
    Two,
}

fn get_diffs(nums: &[i64], part: Part) -> Vec<i64> {
    nums.windows(2)
        .map(|win| match part {
            Part::One => win[1] - win[0],
            Part::Two => win[0] - win[1],
        })
        .collect_vec()
}

fn predict(nums: &[i64], part: Part) -> i64 {
    let mut all_diffs = Vec::new();
    all_diffs.push(nums.to_vec());
    let mut diffs = get_diffs(nums, part);
    all_diffs.push(diffs.clone());

    while diffs.iter().any(|&d| d != 0) {
        diffs = get_diffs(&diffs, part);
        all_diffs.push(diffs.clone());
    }

    for i in (0..all_diffs.len() - 1).rev() {
        let prev = all_diffs[i + 1].last().copied().unwrap();
        let curr = &mut (all_diffs[i]);
        curr.push(
            curr.last().unwrap()
                + match part {
                    Part::One => prev,
                    Part::Two => -prev,
                },
        );
    }

    *all_diffs.first().unwrap().last().unwrap()
}

pub fn part_one(input: &str) -> Option<i64> {
    let nums = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .flat_map(|t| t.parse())
                .collect_vec()
        })
        .collect_vec();
    Some(nums.iter().map(|n| predict(n, Part::One)).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let nums = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .flat_map(|t| t.parse())
                .rev()
                .collect_vec()
        })
        .collect_vec();
    Some(nums.iter().map(|n| predict(n, Part::Two)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
