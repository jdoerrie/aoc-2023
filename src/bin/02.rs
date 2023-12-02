advent_of_code::solution!(2);

struct Bag {
    red: usize,
    green: usize,
    blue: usize,
}

struct Game {
    id: isize,
    bags: Vec<Bag>,
}

fn parse_bag(line: &str) -> Bag {
    let mut bag = Bag {
        red: 0,
        green: 0,
        blue: 0,
    };

    for cube in line.split(", ") {
        let (n, color) = cube.split_once(' ').unwrap();
        let n = n.parse::<usize>().unwrap();
        match color {
            "red" => bag.red = n,
            "green" => bag.green = n,
            "blue" => bag.blue = n,
            _ => panic!(),
        };
    }

    bag
}

fn parse_game(line: &str) -> Game {
    let (game, bags) = line.split_once(": ").unwrap();
    let id: isize = game.split_whitespace().last().unwrap().parse().unwrap();
    let bags: Vec<_> = bags.split("; ").map(parse_bag).collect();
    Game { id, bags }
}

fn is_bag_possible(bag: &Bag) -> bool {
    bag.red <= 12 && bag.green <= 13 && bag.blue <= 14
}

fn is_game_possible(game: &Game) -> bool {
    game.bags.iter().all(is_bag_possible)
}

pub fn part_one(input: &str) -> Option<isize> {
    Some(
        input
            .lines()
            .map(parse_game)
            .filter(is_game_possible)
            .map(|g| g.id)
            .sum(),
    )
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
