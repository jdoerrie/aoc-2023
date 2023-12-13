use itertools::Itertools;

advent_of_code::solution!(13);

type Pattern = Vec<Vec<char>>;
fn parse_pattern(input: &str) -> Pattern {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn transpose(pattern: &Pattern) -> Pattern {
    let mut res = vec![vec![0 as char; pattern.len()]; pattern[0].len()];
    for (i, row) in pattern.iter().enumerate() {
        for (j, e) in row.iter().enumerate() {
            res[j][i] = *e;
        }
    }

    res
}

fn vertical(pattern: &Pattern) -> Vec<usize> {
    vertical_with_filter(pattern, None)
}

fn vertical_with_filter(pattern: &Pattern, filter: Option<usize>) -> Vec<usize> {
    let mut idxs = Vec::new();
    for i in 1..pattern[0].len() {
        if i != filter.unwrap_or(0) {
            idxs.push(i);
        }
    }
    pattern.iter().fold(idxs, |acc, e| {
        acc.into_iter()
            .filter(|&i| e[0..i].iter().rev().zip(e[i..].iter()).all(|(x, y)| x == y))
            .collect()
    })
}

fn horizontal(pattern: &Pattern) -> Vec<usize> {
    vertical(&transpose(pattern))
}

fn horizontal_with_filter(pattern: &Pattern, filter: Option<usize>) -> Vec<usize> {
    vertical_with_filter(&transpose(pattern), filter)
}

fn reflections(pattern: &Pattern) -> [Option<usize>; 2] {
    [
        horizontal(pattern).first().copied(),
        vertical(pattern).first().copied(),
    ]
}

fn reflections_with_filter(
    pattern: &Pattern,
    h: Option<usize>,
    v: Option<usize>,
) -> [Option<usize>; 2] {
    [
        horizontal_with_filter(pattern, h).first().copied(),
        vertical_with_filter(pattern, v).first().copied(),
    ]
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|l| {
                let p = parse_pattern(l);
                let [h, v] = reflections(&p);
                100 * h.unwrap_or(0) + v.unwrap_or(0)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|l| {
                let p = parse_pattern(l);
                let [h, v] = reflections(&p);
                (0..p.len())
                    .cartesian_product(0..p[0].len())
                    .map(|(i, j)| {
                        let mut p = p.clone();
                        p[i][j] = if p[i][j] == '.' { '#' } else { '.' };
                        p
                    })
                    .filter_map(|p| {
                        let [hs, vs] = reflections_with_filter(&p, h, v);
                        if hs.is_none() && vs.is_none() {
                            None
                        } else {
                            Some(100 * hs.unwrap_or(0) + vs.unwrap_or(0))
                        }
                    })
                    .next()
                    .unwrap()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
