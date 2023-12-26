use std::collections::HashMap;

use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};

advent_of_code::solution!(25);

#[derive(Debug)]
struct DisjointSet {
    size: Vec<usize>,
    parent: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> DisjointSet {
        DisjointSet {
            size: vec![1; n],
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);

        if x == y {
            return;
        }

        let (x, y) = if self.size[x] < self.size[y] {
            (x, y)
        } else {
            (y, x)
        };

        self.parent[x] = y;
        self.size[y] += self.size[x];
    }
}

fn min_cut(edges: &mut [(usize, usize)], n: usize) -> usize {
    loop {
        edges.shuffle(&mut thread_rng());
        let mut ds = DisjointSet::new(n);
        let mut cnt = n;

        for &(u, v) in edges.iter() {
            if ds.find(u) != ds.find(v) {
                ds.union(u, v);
                cnt -= 1;
                if cnt == 2 {
                    break;
                }
            }
        }

        let cut = edges
            .iter()
            .filter(|(u, v)| ds.find(*u) != ds.find(*v))
            .copied()
            .collect_vec();
        // println!("cut: {:?}", cut);

        if cut.len() <= 3 {
            let hs = (0..n)
                .map(|i| ds.find(i))
                .fold(HashMap::new(), |mut acc, p| {
                    *acc.entry(p).or_default() += 1;
                    acc
                });
            assert_eq!(hs.len(), 2);
            return hs.values().product();
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let edges = input
        .lines()
        .flat_map(|l| {
            let (u, vs) = l.split_once(": ").unwrap();
            vs.split_ascii_whitespace().map(|v| (u, v)).collect_vec()
        })
        .collect_vec();

    let vertices: HashMap<_, _> = edges
        .iter()
        .flat_map(|(u, v)| [*u, *v])
        .unique()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();

    let mut edges = edges
        .into_iter()
        .map(|(u, v)| (*vertices.get(u).unwrap(), *vertices.get(v).unwrap()))
        .collect_vec();

    Some(min_cut(&mut edges, vertices.len()))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
