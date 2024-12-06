use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Input {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

fn parse(input: &str) -> Input {
    let rules = input
        .lines()
        .take_while(|l| l.contains('|'))
        .map(parse_rule_line)
        .collect();

    let updates = input
        .lines()
        .skip_while(|l| l.contains('|'))
        .skip(1)
        .map(parse_update_line)
        .collect();

    Input { updates, rules }
}

fn parse_update_line(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|w| w.parse().expect("is number"))
        .collect()
}

fn parse_rule_line(input: &str) -> (u32, u32) {
    let mut rule_elem_iter = input.split('|').map(|w| w.parse().expect("is number"));

    (
        rule_elem_iter.next().expect("before"),
        rule_elem_iter.next().expect("after"),
    )
}

enum Rule {
    Before(u32),
    After(u32),
}

struct RuleLookup(HashMap<u32, Vec<Rule>>)

impl RuleLookup {
    fn new(input: &[(u32, u32)]) -> Self {
        let befores = input.iter().map(|(b,x)| (*b, Rule::Before(*x)));
        let afters = input.iter().map(|(x,a)| (*a, Rule::After(*x)));

        Self(befores.concat(afters).group_by(|t| t.0).collect())
    }
}

pub fn is_valid_order(lookup: &RuleLookup, update: &[u32]) -> bool {
    todo!()
}

pub fn middle(v: &[u32]) -> u32 {
    v[v.len() / 2]
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);

    let valid_updates = input.updates.iter().filter(|u| is_valid_order(&input, u));

    let middles = valid_updates.map(|u| middle(u));

    Some(middles.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middle() {
        assert_eq!(middle(&[1, 2, 3]), 2);
        assert_eq!(middle(&[1]), 1);
        assert_eq!(middle(&[1, 2, 3, 4, 5]), 3);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
