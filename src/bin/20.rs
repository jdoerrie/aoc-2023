advent_of_code::solution!(20);

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Pulse {
    High = 0,
    Low = 1,
}

use std::collections::{HashMap, VecDeque};

use Pulse::*;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Module {
    Broadcast,
    FlipFlop(Pulse),
    Conjunction(HashMap<String, Pulse>),
}

use num::Integer;
use Module::*;

fn inv(pulse: Pulse) -> Pulse {
    match pulse {
        Low => High,
        High => Low,
    }
}

fn send(from: &str, pulse: Pulse, module: &mut Module) -> Option<Pulse> {
    match module {
        Broadcast => Some(pulse),
        FlipFlop(fl) => match pulse {
            High => None,
            Low => {
                *fl = inv(*fl);
                Some(*fl)
            }
        },
        Conjunction(inputs) => {
            *inputs.get_mut(from).unwrap() = pulse;
            if inputs.values().all(|&p| p == High) {
                Some(Low)
            } else {
                Some(High)
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut modules = HashMap::new();
    let mut outputs = HashMap::new();
    let mut inputs = HashMap::new();
    for l in input.lines() {
        let (mut name, outs) = l.split_once(" -> ").unwrap();
        match name.as_bytes()[0] {
            b'%' => {
                name = &name[1..];
                modules.insert(name, FlipFlop(Low));
            }
            b'&' => {
                name = &name[1..];
                modules.insert(name, Conjunction(HashMap::new()));
            }
            _ => {
                modules.insert(name, Broadcast);
            }
        };

        for out in outs.split(", ") {
            outputs.entry(name).or_insert(vec![]).push(out);
            inputs.entry(out).or_insert(vec![]).push(name);
        }
    }

    for (name, mdl) in modules.iter_mut() {
        if let Conjunction(map) = mdl {
            *map = inputs
                .get(name)
                .unwrap()
                .iter()
                .map(|i| (i.to_string(), Low))
                .collect();
        }
    }

    let (hi, lo) = (0..1000)
        .map(|_| {
            let mut cnt = HashMap::new();
            let mut q = VecDeque::new();
            q.push_back(("broadcaster", Low));
            while let Some((mdl, in_pulse)) = q.pop_front() {
                for next in outputs.get(mdl).unwrap() {
                    *cnt.entry(in_pulse).or_default() += 1;
                    if let Some(next_mdl) = modules.get_mut(next) {
                        if let Some(out_pulse) = send(mdl, in_pulse, next_mdl) {
                            q.push_back((next, out_pulse));
                        }
                    }
                }
            }

            (
                cnt.get(&High).copied().unwrap_or_default(),
                cnt.get(&Low).copied().unwrap_or_default() + 1,
            )
        })
        .fold((0, 0), |acc, (hi, lo): (usize, usize)| {
            (acc.0 + hi, acc.1 + lo)
        });

    // println!("{hi}, {lo}");
    Some(hi * lo)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut modules = HashMap::new();
    let mut outputs = HashMap::new();
    let mut inputs = HashMap::new();
    for l in input.lines() {
        let (mut name, outs) = l.split_once(" -> ").unwrap();
        match name.as_bytes()[0] {
            b'%' => {
                name = &name[1..];
                modules.insert(name, FlipFlop(Low));
            }
            b'&' => {
                name = &name[1..];
                modules.insert(name, Conjunction(HashMap::new()));
            }
            _ => {
                modules.insert(name, Broadcast);
            }
        };

        for out in outs.split(", ") {
            outputs.entry(name).or_insert(vec![]).push(out);
            inputs.entry(out).or_insert(vec![]).push(name);
        }
    }

    for (name, mdl) in modules.iter_mut() {
        if let Conjunction(map) = mdl {
            *map = inputs
                .get(name)
                .unwrap()
                .iter()
                .map(|i| (i.to_string(), Low))
                .collect();
        }
    }

    let rx_in = *inputs.get("rx")?.first()?;
    let mut first_highs = HashMap::new();

    let mut cnt = 0;
    let mut cnts = HashMap::new();
    let mut found = false;
    while !found {
        cnt += 1;

        let mut q = VecDeque::new();
        q.push_back(("broadcaster", Low));
        while let Some((mdl, in_pulse)) = q.pop_front() {
            for next in outputs.get(mdl).unwrap() {
                if *next == "rdx" && in_pulse == Low {
                    found = true;
                    break;
                }

                if *next == rx_in && in_pulse == High {
                    // println!("{cnt}: Sending {:?} from {mdl} to {next}", in_pulse);
                    first_highs.entry(mdl).or_insert(cnt);
                    if first_highs.len() == inputs.get(rx_in).unwrap().len() {
                        found = true;
                        break;
                    }
                }
                cnts.entry(next).or_insert([0; 2])[in_pulse as usize] += 1;

                if let Some(next_mdl) = modules.get_mut(next) {
                    if let Some(out_pulse) = send(mdl, in_pulse, next_mdl) {
                        q.push_back((next, out_pulse));
                    }
                }
            }
        }
    }

    Some(first_highs.values().fold(1, |acc, e| acc.lcm(e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32000000));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
