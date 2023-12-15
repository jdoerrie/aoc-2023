advent_of_code::solution!(15);

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| ((acc + (c as usize)) * 17) % 256)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.trim_end().split(',').map(hash).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    const EMPTY_VEC: Vec<(&str, usize)> = vec![];
    let mut hash_map = [EMPTY_VEC; 256];
    for t in input.trim_end().split(',') {
        if let Some(i) = t.find('=') {
            let (label, num) = t.split_at(i);
            let num = num.strip_prefix('=').unwrap().parse().unwrap();
            let v = &mut hash_map[hash(label)];
            if let Some(p) = v.iter().position(|(l, _)| *l == label) {
                v[p].1 = num;
            } else {
                v.push((label, num));
            }
        } else {
            let label = t.strip_suffix('-').unwrap();
            let v = &mut hash_map[hash(label)];
            if let Some(p) = v.iter().position(|(l, _)| *l == label) {
                v.remove(p);
            }
        }
    }

    Some(
        hash_map
            .into_iter()
            .enumerate()
            .map(|(k, v)| {
                v.iter()
                    .enumerate()
                    .map(|(i, (_, l))| (k + 1) * (i + 1) * l)
                    .sum::<usize>()
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
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
