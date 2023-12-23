use itertools::Itertools;
use ndarray::{s, Array3};
use tuple::Map;

advent_of_code::solution!(22);

fn parse(input: &str) -> Array3<usize> {
    let ranges = input
        .lines()
        .map(|l| {
            let (lhs, rhs) = l.split_once('~').unwrap().map(|s| {
                s.split(',')
                    .filter_map(|n| n.parse::<usize>().ok())
                    .collect_vec()
            });

            [lhs[0]..rhs[0] + 1, lhs[1]..rhs[1] + 1, lhs[2]..rhs[2] + 1]
        })
        .collect_vec();

    let maxs = ranges.iter().cloned().fold([0; 3], |[x, y, z], [i, j, k]| {
        [x.max(i.end), y.max(j.end), z.max(k.end)]
    });

    let mut space = ndarray::Array::zeros(maxs);
    for (i, [x, y, z]) in ranges.into_iter().enumerate() {
        space.slice_mut(s![x, y, z]).fill(i + 1);
    }

    space
}

fn fall_sand(blocks: &Array3<usize>) -> Array3<usize> {
    let mut res = Array3::zeros(blocks.shape().clone());
    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let space = parse(input);
    println!("{:?}", space);
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
