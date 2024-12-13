use std::{collections::HashMap, mem};

use itertools::Itertools;

advent_of_code::solution!(11);

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|num| num.parse().expect("number can be parsed"))
        .collect()
}

struct Solution(HashMap<usize, usize>);

impl Solution {
    fn new(stones: &[usize]) -> Self {
        Self(stones.iter().cloned().counts())
    }

    fn blink_times(&mut self, blinks: usize) {
        for _ in 0..blinks {
            self.blink_once();
        }
    }

    fn blink_once(&mut self) {
        let mut prev = HashMap::new();
        mem::swap(&mut self.0, &mut prev);

        for (stone, count) in prev.into_iter() {
            if stone == 0 {
                self.0.entry(1).and_modify(|c| *c += count).or_insert(count);
                continue;
            }
            let digits = stone.to_string();
            if digits.len() % 2 == 0 {
                let (first, second) = digits.split_at(digits.len() / 2);
                let (first, second) = (
                    first.parse().expect("number"),
                    second.parse().expect("number"),
                );
                self.0
                    .entry(first)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
                self.0
                    .entry(second)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
                continue;
            }
            self.0
                .entry(stone * 2024)
                .and_modify(|c| *c += count)
                .or_insert(count);
        }
    }

    fn result(&self) -> usize {
        self.0.values().sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut solution = Solution::new(&parse(input));
    solution.blink_times(25);
    Some(solution.result())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut solution = Solution::new(&parse(input));
    solution.blink_times(75);
    Some(solution.result())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
