advent_of_code::solution!(18);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

use itertools::Itertools;
use Dir::*;

fn parse_dirs(input: &str) -> Vec<(Dir, usize)> {
    input
        .lines()
        .map(|l| {
            let (d, n, _) = l.split_ascii_whitespace().collect_tuple().unwrap();
            (
                match d {
                    "D" => Down,
                    "U" => Up,
                    "L" => Left,
                    "R" => Right,
                    _ => unreachable!("Wrong Dir"),
                },
                n.parse().unwrap(),
            )
        })
        .collect()
}

fn parse_colors(input: &str) -> Vec<(Dir, usize)> {
    input
        .lines()
        .map(|l| {
            let (_, _, hex) = l.split_ascii_whitespace().collect_tuple().unwrap();
            (
                match hex.as_bytes()[7] {
                    b'0' => Right,
                    b'1' => Down,
                    b'2' => Left,
                    b'3' => Up,
                    _ => unreachable!("Wrong Dir"),
                },
                usize::from_str_radix(&hex[2..7], 16).unwrap(),
            )
        })
        .collect()
}

fn shoelace(points: &[[isize; 2]]) -> isize {
    let mut acc = 0;
    for i in 0..points.len() {
        let [x1, y1] = points[i];
        let [x2, y2] = points[(i + 1) % points.len()];
        let res = x1 * y2 - x2 * y1;
        acc += res;
        acc += (x1.abs_diff(x2) + y1.abs_diff(y2)) as isize;
    }
    acc.abs() / 2 + 1
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut points = vec![];
    let (mut r, mut c) = (0, 0);
    for (dir, cnt) in parse_dirs(input) {
        let cnt = (cnt) as isize;
        let delta = match dir {
            Up => (-cnt, 0),
            Down => (cnt, 0),
            Left => (0, -cnt),
            Right => (0, cnt),
        };

        r += delta.0;
        c += delta.1;
        points.push([r, c]);
    }

    points.reverse();
    Some(shoelace(&points) as usize)
}

pub fn part_two(input: &str) -> Option<usize> {
    let dirs = parse_colors(input);
    let [mut x, mut y] = [0, 0];
    let mut points = vec![];
    for (dir, cnt) in dirs {
        let cnt = cnt as isize;
        match dir {
            Up => x -= cnt,
            Down => x += cnt,
            Left => y -= cnt,
            Right => y += cnt,
        };

        points.push([x, y]);
    }

    points.reverse();
    Some(shoelace(&points) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
