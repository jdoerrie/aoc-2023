use itertools::Itertools;

advent_of_code::solution!(11);

type Coords = [usize; 2];

fn parse_galaxies(input: &str) -> Vec<Coords> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.char_indices()
                .filter_map(move |(j, c)| if c == '#' { Some([i, j]) } else { None })
        })
        .collect()
}

fn unique_rows(coords: &[Coords]) -> Vec<usize> {
    coords.iter().map(|[i, _]| *i).sorted().unique().collect()
}

fn unique_cols(coords: &[Coords]) -> Vec<usize> {
    coords.iter().map(|[_, j]| *j).sorted().unique().collect()
}

fn adjust_galaxies(
    galaxies: &[Coords],
    rows: &[usize],
    cols: &[usize],
    scale: usize,
) -> Vec<Coords> {
    galaxies
        .iter()
        .map(|&[i, j]| {
            [
                i + (scale - 1) * (i + 1 - rows.partition_point(|&r| r < i)),
                j + (scale - 1) * (j + 1 - cols.partition_point(|&c| c < j)),
            ]
        })
        .collect()
}

pub fn manhatten_distance([x1, x2]: &Coords, [y1, y2]: &Coords) -> usize {
    x1.abs_diff(*y1) + x2.abs_diff(*y2)
}

pub fn part_one(input: &str) -> Option<usize> {
    let galaxies = parse_galaxies(input);

    let rows = unique_rows(&galaxies);
    let cols = unique_cols(&galaxies);

    let adjusted_galaxies = adjust_galaxies(&galaxies, &rows, &cols, 2);
    Some(
        adjusted_galaxies
            .iter()
            .combinations(2)
            .map(|c| manhatten_distance(c[0], c[1]))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let galaxies = parse_galaxies(input);

    let rows = unique_rows(&galaxies);
    let cols = unique_cols(&galaxies);

    let adjusted_galaxies = adjust_galaxies(&galaxies, &rows, &cols, 1000000);
    Some(
        adjusted_galaxies
            .iter()
            .combinations(2)
            .map(|c| manhatten_distance(c[0], c[1]))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
