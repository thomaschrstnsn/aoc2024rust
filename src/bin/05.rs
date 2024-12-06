use std::collections::{HashMap, HashSet};

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

struct RuleLookup {
    befores: HashMap<u32, HashSet<u32>>,
    afters: HashMap<u32, HashSet<u32>>,
}

impl RuleLookup {
    fn new(input: &[(u32, u32)]) -> Self {
        let mut befores = HashMap::new();
        for (k, v) in &input.iter().map(|(b, a)| (*a, *b)).chunk_by(|(b, _)| *b) {
            let v: HashSet<u32> = v.map(|t| t.1).collect();
            befores.insert(k, v);
        }

        let mut afters = HashMap::new();
        for (k, v) in &input.iter().map(|(b, a)| (*b, *a)).chunk_by(|(b, _)| *b) {
            let v: HashSet<u32> = v.map(|t| t.1).collect();
            afters.insert(k, v);
        }

        Self { befores, afters }
    }

    fn invalid_order(&self, update: &[u32]) -> Option<(u32, u32)> {
        let mut rest: HashSet<u32> = HashSet::from_iter(update.iter().copied());
        let mut seen: HashSet<u32> = HashSet::new();
        for n in update {
            if let Some(befores) = self.befores.get(n) {
                if let Some(should_be_before) = befores.intersection(&rest).next() {
                    return Some((*should_be_before, *n));
                }
            }
            if let Some(afters) = self.afters.get(n) {
                if let Some(should_be_after) = afters.intersection(&seen).next() {
                    return Some((*n, *should_be_after));
                }
            }

            rest.remove(n);
            seen.insert(*n);
        }
        None
    }

    fn is_valid_order(&self, update: &[u32]) -> bool {
        self.invalid_order(update).is_none()
    }

    fn fix_ordering(&self, update: &[u32]) -> Vec<u32> {
        let mut res = update.to_vec();

        while let Some((a, b)) = self.invalid_order(&res) {
            let a_index = res.iter().position(|x| *x == a).unwrap();
            let b_index = res.iter().position(|x| *x == b).unwrap();
            res.swap(a_index, b_index);
        }

        res
    }
}

pub fn middle(v: &[u32]) -> u32 {
    v[v.len() / 2]
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);

    let lookup = RuleLookup::new(&input.rules);

    let valid_updates = input.updates.iter().filter(|x| lookup.is_valid_order(x));

    let middles = valid_updates.map(|u| middle(u));

    Some(middles.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);

    let lookup = RuleLookup::new(&input.rules);

    let invalid_updates = input.updates.iter().filter(|u| !lookup.is_valid_order(u));

    // println!("broken {:?}", invalid_updates.collect::<Vec<_>>());

    let fixed_updates = invalid_updates.map(|iu| lookup.fix_ordering(iu));

    // println!("fixed {:?}", invalid_updates..collect::<Vec<_>>());

    let middles = fixed_updates.map(|u| middle(&u));

    Some(middles.sum())
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
        assert_eq!(result, Some(123));
    }
}
