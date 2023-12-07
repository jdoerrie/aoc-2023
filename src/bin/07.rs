use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Ord, Eq, PartialEq, PartialOrd, Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    cards: String,
    bid: i64,
}

type FrequencyTable = HashMap<char, usize>;

fn get_freqs(cards: &str) -> FrequencyTable {
    let mut freqs = FrequencyTable::new();
    for c in cards.chars() {
        *freqs.entry(c).or_default() += 1;
    }
    freqs
}

fn get_ranks(card_ranks: &str, cards: &str) -> Vec<usize> {
    cards.chars().flat_map(|c| card_ranks.find(c)).collect_vec()
}

fn get_type(freqs: &FrequencyTable) -> Type {
    let rev_freqs = freqs.iter().map(|(_, &v)| v).sorted().rev().collect_vec();
    match (rev_freqs[0], rev_freqs.get(1).copied().unwrap_or_default()) {
        (5, _) => Type::FiveOfAKind,
        (4, _) => Type::FourOfAKind,
        (3, 2) => Type::FullHouse,
        (3, _) => Type::ThreeOfAKind,
        (2, 2) => Type::TwoPair,
        (2, _) => Type::OnePair,
        _ => Type::HighCard,
    }
}

fn apply_joker(freqs: &mut FrequencyTable) {
    let cnt = freqs.remove(&'J').unwrap_or_default();
    if let Some(max_val) = freqs.values_mut().max() {
        *max_val += cnt;
    } else {
        freqs.insert('J', cnt);
    }
}

fn parse_hand(line: &str) -> Hand {
    let (cards, bid) = line.split_once(' ').unwrap();
    let bid = bid.parse().unwrap();
    Hand {
        cards: String::from(cards),
        bid,
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let hands = input.lines().map(parse_hand).collect_vec();
    Some(
        hands
            .iter()
            .map(|h| {
                let freqs = get_freqs(&h.cards);
                (
                    get_type(&freqs),
                    get_ranks("23456789TJQKA", &h.cards),
                    h.bid,
                )
            })
            .sorted()
            .enumerate()
            .map(|(i, (_, _, bid))| (i + 1) as i64 * bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let hands = input.lines().map(parse_hand).collect_vec();
    Some(
        hands
            .iter()
            .map(|h| {
                let mut freqs = get_freqs(&h.cards);
                apply_joker(&mut freqs);
                (
                    get_type(&freqs),
                    get_ranks("J23456789TQKA", &h.cards),
                    h.bid,
                )
            })
            .sorted()
            .enumerate()
            .map(|(i, (_, _, bid))| (i + 1) as i64 * bid)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
