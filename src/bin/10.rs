use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use phf::phf_map;

advent_of_code::solution!(10);

#[derive(Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

type Pipe = [Dir; 2];

static PIPES: phf::Map<char, Pipe> = phf_map! {
    '|' => [Dir::North, Dir::South],
    '-' => [Dir::East, Dir::West],
    'L' => [Dir::North, Dir::East],
    'J' => [Dir::North, Dir::West],
    '7' => [Dir::South, Dir::West],
    'F' => [Dir::South, Dir::East],
};

type Coord = [usize; 2];
type Neighbors = [Coord; 2];

fn step(coord: Coord, dir: Dir) -> Coord {
    let [x, y] = coord;
    match dir {
        Dir::North => [x.max(1) - 1, y],
        Dir::South => [x + 1, y],
        Dir::East => [x, y + 1],
        Dir::West => [x, y.max(1) - 1],
    }
}

fn get_start(input: &str) -> Coord {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.char_indices()
                .filter_map(move |(j, c)| if c == 'S' { Some([i, j]) } else { None })
        })
        .next()
        .unwrap()
}

fn get_network(input: &str, start: Coord) -> HashMap<Coord, Neighbors> {
    let mut network: HashMap<Coord, Neighbors> =
        HashMap::from_iter(input.lines().enumerate().flat_map(|(i, l)| {
            l.char_indices().filter_map(move |(j, c)| {
                PIPES.get(&c).map(|p| ([i, j], p.map(|d| step([i, j], d))))
            })
        }));

    let start_neighbors: Neighbors = network
        .iter()
        .filter_map(|(k, v)| if v.contains(&start) { Some(*k) } else { None })
        .collect_vec()
        .try_into()
        .unwrap();
    network.insert(start, start_neighbors);
    network
}

pub fn part_one(input: &str) -> Option<u32> {
    let start = get_start(input);
    let network = get_network(input, start);

    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    let mut max_i = 0;
    while let Some((c, i)) = q.pop_front() {
        max_i = max_i.max(i);
        visited.insert(c);
        q.extend(network.get(&c).unwrap().iter().filter_map(|next| {
            if !visited.contains(next) {
                Some((*next, i + 1))
            } else {
                None
            }
        }));
    }

    Some(max_i)
}

type Grid = Vec<Vec<i16>>;

fn get_offsets(c: char) -> Vec<(usize, usize)> {
    match c {
        '|' => vec![(1, 0), (1, 1), (1, 2)],
        '-' => vec![(0, 1), (1, 1), (2, 1)],
        'L' => vec![(1, 0), (1, 1), (2, 1)],
        'J' => vec![(1, 0), (0, 1), (1, 1)],
        'F' => vec![(1, 1), (2, 1), (1, 2)],
        '7' => vec![(0, 1), (1, 1), (1, 2)],
        'S' => (0..3).cartesian_product(0..3).collect_vec(),
        _ => panic!("Unsupported"),
    }
}

fn build_grid(input: &str, visited: &HashSet<Coord>) -> Grid {
    let scale = 3;
    let chars = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut grid = vec![vec![0; chars[0].len() * scale]; chars.len() * scale];
    for &[i, j] in visited {
        for (y, x) in get_offsets(chars[i][j]) {
            grid[scale * i + x][scale * j + y] = 1;
        }
    }
    grid
}

fn color_grid_impl(grid: &mut Grid, color: i16, i: isize, j: isize) -> bool {
    let mut stack = VecDeque::new();
    stack.push_back((i, j));

    let mut all = true;
    while let Some((i, j)) = stack.pop_back() {
        if i < 0
            || j < 0
            || grid.get(i as usize).is_none()
            || grid[i as usize].get(j as usize).is_none()
        {
            all = false;
            continue;
        }

        let cell = &mut grid[i as usize][j as usize];
        if *cell > 0 {
            continue;
        }

        *cell = color;
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            stack.push_back((i + dx, j + dy));
        }
    }
    all
}

fn color_grid(grid: &mut Grid) -> usize {
    let mut color = 2;
    let mut good_colors = HashSet::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                if color_grid_impl(grid, color, i as isize, j as isize) {
                    good_colors.insert(color);
                }
                color += 1;
            }
        }
    }

    let mut inside = 0;
    for i in 0..grid.len() / 3 {
        for j in 0..grid[i].len() / 3 {
            let (i3, j3) = (i * 3, j * 3);
            if good_colors.contains(&grid[i3][j3])
                && (0..3)
                    .cartesian_product(0..3)
                    .all(|(di, dj)| grid[i3][j3] == grid[i3 + di][j3 + dj])
            {
                inside += 1;
            }
        }
    }

    inside
}

pub fn part_two(input: &str) -> Option<usize> {
    let start = get_start(input);
    let network = get_network(input, start);

    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(start);
    while let Some(c) = q.pop_front() {
        visited.insert(c);
        q.extend(network.get(&c).unwrap().iter().filter_map(|next| {
            if !visited.contains(next) {
                Some(*next)
            } else {
                None
            }
        }));
    }

    let mut grid = build_grid(input, &visited);
    Some(color_grid(&mut grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }
}
