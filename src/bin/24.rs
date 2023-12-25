use itertools::Itertools;

advent_of_code::solution!(24);

#[derive(Clone, Debug)]
struct Line {
    pos: [i128; 3],
    vel: [i128; 3],
}

// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
fn intersect_xy(lhs: &Line, rhs: &Line) -> Option<[f64; 2]> {
    let [x1, y1] = [lhs.pos[0], lhs.pos[1]];
    let [x2, y2] = [x1 + lhs.vel[0], y1 + lhs.vel[1]];
    let [x3, y3] = [rhs.pos[0], rhs.pos[1]];
    let [x4, y4] = [x3 + rhs.vel[0], y3 + rhs.vel[1]];

    let px_num = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
    let py_num = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);

    let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    if den != 0 {
        Some([px_num as f64 / den as f64, py_num as f64 / den as f64])
    } else {
        None
    }
}

fn intersect_time(lhs: &Line, rhs: &Line) -> Option<[f64; 2]> {
    intersect_xy(lhs, rhs).map(|[px, _]| {
        [
            (px - lhs.pos[0] as f64) / lhs.vel[0] as f64,
            (px - rhs.pos[0] as f64) / rhs.vel[0] as f64,
        ]
    })
}

fn parse(line: &str) -> Line {
    let (pos, vel) = line.split_once(" @ ").unwrap();
    Line {
        pos: pos
            .split(',')
            .filter_map(|n| n.trim().parse().ok())
            .collect_vec()
            .try_into()
            .unwrap(),
        vel: vel
            .split(',')
            .filter_map(|n| n.trim().parse().ok())
            .collect_vec()
            .try_into()
            .unwrap(),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let min = 200_000_000_000_000.;
    let max = 400_000_000_000_000.;
    Some(
        input
            .lines()
            .map(parse)
            .tuple_combinations()
            .filter(|(lhs, rhs)| {
                let xy = intersect_xy(lhs, rhs);
                let time = intersect_time(lhs, rhs);
                xy.is_some_and(|[x, y]| min <= x && x <= max && min <= y && y <= max)
                    && time.is_some_and(|[x, y]| x >= 0. && y >= 0.)
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // l1 = 19, 13, 30 @ -2,  1, -2
    // l2 = 18, 19, 22 @ -1, -1, -2
    // 20, 25, 34 @ -2, -2, -4
    // 12, 31, 28 @ -1, -2, -1
    // 20, 19, 15 @  1, -5, -3

    let p = (19, 13, 30);
    let q = (18, 19, 22);
    let v = (-2, 1, -2);
    let pq = (p.0 - q.0, p.1 - q.1, p.2 - q.2);

    // Normal of Plane through l1 and a point on l2.
    let pq_x_v = (
        pq.1 * v.2 - v.1 * pq.2,
        pq.2 * v.0 - v.2 * pq.0,
        pq.0 * v.1 - v.0 * pq.1,
    );

    


    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
