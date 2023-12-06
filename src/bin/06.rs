use itertools::Itertools;
use tuple::Map;

advent_of_code::solution!(6);

fn parse_vals(line: &str) -> Vec<f64> {
    line.split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (times, distances) = input.lines().collect_tuple().unwrap();
    let (times, distances) = (times, distances).map(parse_vals);

    Some(
        times
            .into_iter()
            .zip(distances)
            .map(|(t, d)| {
                let min_t = t / 2.0 - (f64::sqrt(t * t / 4.0 - d).max(0.0));
                let ceil_t = (min_t + 1e-8).ceil() as u64;
                (t as u64 - 2 * ceil_t + 1).max(0)
            })
            .product::<u64>(),
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
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
