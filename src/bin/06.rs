use itertools::Itertools;
use tuple::Map;

advent_of_code::solution!(6);

fn parse_vals(line: &str) -> Vec<f64> {
    line.split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect_vec()
}

fn parse_single_val(line: &str) -> f64 {
    line.chars().fold(0.0, |acc, c| {
        if c.is_ascii_digit() {
            acc * 10.0 + (c.to_digit(10).unwrap_or(0)) as f64
        } else {
            acc
        }
    })
}

fn get_n_times(t: f64, d: f64) -> u64 {
    let min_t = t / 2.0 - (f64::sqrt(t * t / 4.0 - d).max(0.0));
    let ceil_t = (min_t + 1e-8).ceil() as u64;
    (t as u64 - 2 * ceil_t + 1).max(0)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (times, distances) = input.lines().collect_tuple().unwrap();
    let (times, distances) = (times, distances).map(parse_vals);

    Some(
        times
            .into_iter()
            .zip(distances)
            .map(|(t, d)| get_n_times(t, d))
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
