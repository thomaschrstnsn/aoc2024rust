use itertools::{sorted, Itertools};

advent_of_code::solution!(1);

fn parse(input: &str) -> Vec<(usize, usize)> {
    input.lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> (usize, usize) {
    let mut iter = input
        .split_whitespace()
        .map(|num| num.parse().expect("can parse"));

    (iter.next().unwrap(), iter.next().unwrap())
}

pub fn part_one(input: &str) -> Option<usize> {
    let parsed = parse(input);

    let left = parsed.iter().map(|t| t.0);
    let right = parsed.iter().map(|t| t.1);

    Some(
        sorted(left)
            .zip(sorted(right))
            .map(|(left, right)| left.abs_diff(right))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let parsed = parse(input);

    let left = parsed.iter().map(|t| t.0);
    let right = parsed.iter().map(|t| t.1).counts();

    Some(left.map(|n| right.get(&n).unwrap_or(&0) * n).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
