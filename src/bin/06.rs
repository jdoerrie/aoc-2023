use itertools::Itertools;
use tuple::Map;

advent_of_code::solution!(6);

fn parse_vals(line: &str) -> Vec<f64> {
    line.split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect_vec()
}

fn parse_single_val(line: &str) -> f64 {
    line.chars()
        .filter_map(|c| c.to_digit(10).map(f64::from))
        .fold(0.0, |acc, d| acc * 10.0 + d)
}

fn get_n_times(time: f64, dist: f64) -> u64 {
    let min_time = time / 2.0 - (f64::sqrt(time * time / 4.0 - dist).max(0.0));
    let ceil_time = (min_time + 1e-8).ceil() as u64;
    (time as u64 - 2 * ceil_time + 1).max(0)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (times, distances) = input.lines().collect_tuple().unwrap();
    let (times, distances) = (times, distances).map(parse_vals);

    Some(
        times
            .into_iter()
            .zip(distances)
            .map(|(time, dist)| get_n_times(time, dist))
            .product::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (time, distance) = input.lines().collect_tuple().unwrap();
    let (time, distance) = (time, distance).map(parse_single_val);
    Some(get_n_times(time, distance))
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
        assert_eq!(result, Some(71503));
    }
}
