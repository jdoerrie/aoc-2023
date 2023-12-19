advent_of_code::solution!(19);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Category {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

use std::{collections::HashMap, ops::Range};

use Category::*;

#[derive(Debug)]
enum Compare {
    LessThan,
    GreaterThan,
}
#[derive(Debug)]
struct Rule {
    cat: Category,
    cmp: Compare,
    lim: usize,
}

const ALWAYS_TRUE: Rule = Rule {
    cat: X,
    cmp: Compare::LessThan,
    lim: usize::MAX,
};

#[derive(Debug)]
struct WorkFlow {
    rules: Vec<(Rule, Flow)>,
}

#[derive(Debug, Clone, PartialEq)]
enum Flow {
    Work(String),
    Accepted,
    Rejected,
}

use itertools::Itertools;
use Flow::*;

fn parse_flow(input: &str) -> Flow {
    match input {
        "A" => Accepted,
        "R" => Rejected,
        _ => Work(input.to_string()),
    }
}

fn parse_cat(input: &str) -> Category {
    match input.as_bytes()[0] {
        b'x' => X,
        b'm' => M,
        b'a' => A,
        b's' => S,
        _ => unreachable!("Unknown Category"),
    }
}

fn parse_rule(input: &str) -> Rule {
    if input.is_empty() {
        return ALWAYS_TRUE;
    }

    if let Some((cat, lim)) = input.split_once('<') {
        Rule {
            cat: parse_cat(cat),
            cmp: Compare::LessThan,
            lim: lim.parse().unwrap(),
        }
    } else {
        let (cat, lim) = input.split_once('>').unwrap();
        Rule {
            cat: parse_cat(cat),
            cmp: Compare::GreaterThan,
            lim: lim.parse().unwrap(),
        }
    }
}

type Flows = HashMap<String, WorkFlow>;

fn parse_flows(input: &str) -> Flows {
    input
        .lines()
        .map(|l| {
            let (name, rules) = l.split_once('{').unwrap();
            let rules = rules.strip_suffix('}').unwrap();
            (
                name.to_string(),
                WorkFlow {
                    rules: {
                        rules
                            .split(',')
                            .map(|rule| {
                                let (rule, next) = rule.split_once(':').unwrap_or(("", rule));
                                (parse_rule(rule), parse_flow(next))
                            })
                            .collect_vec()
                    },
                },
            )
        })
        .collect()
}

type Item = [usize; 4];
fn parse_items(input: &str) -> Vec<Item> {
    input
        .lines()
        .map(|l| {
            let l = l
                .strip_prefix('{')
                .and_then(|l| l.strip_suffix('}'))
                .unwrap();
            l.split(',')
                .map(|item| {
                    let (cat, lim) = item.split_once('=').unwrap();
                    (parse_cat(cat) as u8, lim.parse().unwrap())
                })
                .sorted()
                .map(|(_, x)| x)
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn eval_rule(item: &Item, rule: &Rule) -> bool {
    match rule.cmp {
        Compare::LessThan => item[rule.cat as usize] < rule.lim,
        Compare::GreaterThan => item[rule.cat as usize] > rule.lim,
    }
}

fn eval_flow(item: &Item, flows: &WorkFlow) -> Flow {
    flows
        .rules
        .iter()
        .find(|(rule, _)| eval_rule(item, rule))
        .unwrap()
        .1
        .clone()
}

fn eval_flows(item: &Item, flows: &Flows) -> Flow {
    let mut rule = "in".to_string();
    while let Work(str) = eval_flow(item, flows.get(&rule).unwrap()) {
        rule = str;
    }

    eval_flow(item, flows.get(&rule).unwrap())
}

pub fn part_one(input: &str) -> Option<usize> {
    let (flows, items) = input.split_once("\n\n").unwrap();
    let flows = parse_flows(flows);
    let items = parse_items(items);

    Some(
        items
            .iter()
            .filter(|item| eval_flows(item, &flows) == Accepted)
            .map(|item| item.iter().sum::<usize>())
            .sum(),
    )
}

type Ranges = [Range<usize>; 4];

fn n_items(rng: &Ranges) -> usize {
    rng.iter().map(|r| r.end - r.start.min(r.end)).product()
}

fn split_ranges_at_rule(ranges: &Ranges, rule: &Rule) -> (Ranges, Ranges) {
    let (mut lhs, mut rhs) = (ranges.clone(), ranges.clone());

    let rng = ranges[rule.cat as usize].clone();
    let (min, max) = match rule.cmp {
        Compare::LessThan => (
            rng.start..rng.end.min(rule.lim),
            rng.start.max(rule.lim)..rng.end,
        ),
        Compare::GreaterThan => (
            rng.start.max(rule.lim + 1)..rng.end,
            rng.start..rng.end.min(rule.lim + 1),
        ),
    };

    lhs[rule.cat as usize] = min;
    rhs[rule.cat as usize] = max;
    assert_eq!(n_items(ranges), n_items(&lhs) + n_items(&rhs));
    (lhs, rhs)
}

fn split_ranges(work_flow: &WorkFlow, ranges: &Ranges) -> Vec<(Flow, Ranges)> {
    let mut ranges = ranges.clone();
    work_flow
        .rules
        .iter()
        .map(|(rule, flow)| {
            let (passing, failing) = split_ranges_at_rule(&ranges, rule);
            ranges = failing;
            (flow.clone(), passing)
        })
        .collect()
}

fn solve(flows: &Flows) -> usize {
    let ranges = [1..4001, 1..4001, 1..4001, 1..4001];
    let mut accepted = vec![];
    let mut stack = vec![("in".to_string(), ranges)];
    while let Some((name, rngs)) = stack.pop() {
        for (flow, rng) in split_ranges(flows.get(&name).unwrap(), &rngs) {
            match flow {
                Accepted => accepted.push(rng),
                Work(name) => stack.push((name, rng)),
                Rejected => {}
            }
        }
    }

    accepted.iter().map(n_items).sum()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (flows, _) = input.split_once("\n\n").unwrap();
    let flows = parse_flows(flows);
    Some(solve(&flows))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
