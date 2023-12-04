use std::collections::HashSet;

advent_of_code::solution!(4);

fn split_into_set(input: &str) -> HashSet<u32> {
    input
        .split_ascii_whitespace()
        .flat_map(|n| n.parse())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (winning_numbers, scratch_card) =
                    line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
                (match split_into_set(winning_numbers)
                    .intersection(&split_into_set(scratch_card))
                    .count()
                {
                    0 => 0,
                    i => 1 << (i - 1),
                }) as u32
            })
            .sum(),
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
