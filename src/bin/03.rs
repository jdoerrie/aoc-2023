use itertools::Itertools;

advent_of_code::solution!(3);

struct Symbol {
    c: char,
    x: usize,
    y: usize,
}

struct Number {
    n: u32,
    x: usize,
    y_beg: usize,
    y_end: usize,
}

fn parse_symbols(input: &str) -> Vec<Symbol> {
    let mut symbols = Vec::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.char_indices() {
            if c != '.' && !c.is_ascii_digit() {
                symbols.push(Symbol { c, x, y });
            }
        }
    }

    symbols
}

fn parse_numbers(input: &str) -> Vec<Number> {
    let mut numbers = Vec::new();
    for (x, line) in input.lines().enumerate() {
        for (_, group) in line
            .char_indices()
            .group_by(|(_, c)| c.is_ascii_digit())
            .into_iter()
            .filter(|(key, _)| *key)
        {
            let mut n = 0;
            let mut y_beg = usize::MAX;
            let mut y_end = usize::MIN;
            for (i, c) in group {
                n *= 10;
                n += c.to_digit(10).unwrap();
                y_beg = y_beg.min(i);
                y_end = y_end.max(i);
            }

            numbers.push(Number { n, x, y_beg, y_end });
        }
    }

    numbers
}

fn is_adjacent(s: &Symbol, n: &Number) -> bool {
    s.x.abs_diff(n.x) <= 1 && n.y_beg <= s.y + 1 && s.y <= n.y_end + 1
}

pub fn part_one(input: &str) -> Option<u32> {
    let symbols = parse_symbols(input);
    let numbers = parse_numbers(input);
    Some(
        numbers
            .iter()
            .filter(|n| symbols.iter().any(|s| is_adjacent(s, n)))
            .map(|n| n.n)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let symbols = parse_symbols(input)
        .into_iter()
        .filter(|s| s.c == '*')
        .collect_vec();
    let numbers = parse_numbers(input);

    let mut gear_ratios = 0;
    for s in symbols {
        let nums = numbers.iter().filter(|n| is_adjacent(&s, n)).collect_vec();
        if nums.len() == 2 {
            gear_ratios += nums[0].n * nums[1].n;
        }
    }

    Some(gear_ratios)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
