use itertools::Itertools;
use ndarray::{s, Array2};
use std::ops::Range;
use tuple::Map;

advent_of_code::solution!(22);

type Range3 = [Range<usize>; 3];

fn parse(input: &str) -> Vec<Range3> {
    input
        .lines()
        .map(|l| {
            let (lhs, rhs) = l.split_once('~').unwrap().map(|s| {
                s.split(',')
                    .filter_map(|n| n.parse::<usize>().ok())
                    .collect_vec()
            });

            [lhs[0]..rhs[0] + 1, lhs[1]..rhs[1] + 1, lhs[2]..rhs[2] + 1]
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut ranges = parse(input);
    ranges.sort_by(|x, y| x[2].start.cmp(&y[2].start));

    let mut adj = vec![vec![]; ranges.len()];
    let mut depth_map = Array2::from_elem((10, 10), (1, None));
    for (i, rng) in ranges.iter().enumerate() {
        let mut slice = depth_map.slice_mut(s![rng[0].clone(), rng[1].clone()]);
        let max_depth = slice.iter().max().unwrap().0;
        let touching = slice
            .iter()
            .filter_map(|(dep, block)| if *dep == max_depth { *block } else { None })
            .unique();
        adj[i].extend(touching);
        slice.fill((rng[2].end - rng[2].start + max_depth, Some(i)));
        println!("Block: {:?}, Final Depth: {max_depth}", (i, rng));
    }

    Some(
        ranges.len()
            - adj
                .iter()
                .filter_map(|v| if v.len() == 1 { Some(v[0]) } else { None })
                .unique()
                .count(),
    )
}

fn get_final_depths(ranges: &[Option<Range3>]) -> Vec<Option<usize>> {
    let mut map = vec![];
    let mut depth_map = Array2::from_elem((10, 10), 1);
    for rng in ranges {
        if rng.is_none() {
            map.push(None);
            continue;
        }

        let [x_rng, y_rng, z_rng] = rng.clone().unwrap();
        let mut slice = depth_map.slice_mut(s![x_rng, y_rng]);
        let max_depth = *slice.iter().max().unwrap();
        slice.fill(z_rng.end - z_rng.start + max_depth);
        map.push(Some(max_depth));
    }

    map
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut ranges = parse(input);
    ranges.sort_by(|x, y| x[2].start.cmp(&y[2].start));
    let mut opt_ranges = ranges.iter().map(|rng| Some(rng.clone())).collect_vec();

    let mut diffs = vec![];
    let orig_depths = get_final_depths(&opt_ranges);
    for i in 0..opt_ranges.len() {
        let orig = opt_ranges[i].clone();
        opt_ranges[i] = None;
        let depth = get_final_depths(&opt_ranges);
        opt_ranges[i] = orig;

        diffs.push(
            orig_depths
                .iter()
                .zip(depth)
                .filter(|(x, y)| {
                    if x.is_none() || y.is_none() {
                        false
                    } else {
                        x.unwrap() != y.unwrap()
                    }
                })
                .count(),
        );
    }

    Some(diffs.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
