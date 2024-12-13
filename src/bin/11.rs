advent_of_code::solution!(11);

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|num| num.parse().expect("number can be parsed"))
        .collect()
}

fn blink_one(n: usize) -> Vec<usize> {
    if n == 0 {
        return vec![1];
    }
    let digits = n.to_string();
    if digits.len() % 2 == 0 {
        let (first, second) = digits.split_at(digits.len() / 2);
        let (first, second) = (
            first.parse().expect("number"),
            second.parse().expect("number"),
        );
        return vec![first, second];
    }
    vec![n * 2024]
}

fn blink_once(nums: &[usize]) -> impl Iterator<Item = usize> + use<'_> {
    nums.iter().flat_map(|num| blink_one(*num))
}

fn blink(mut nums: Vec<usize>, iters: usize) -> Vec<usize> {
    for _ in 0..iters {
        nums = blink_once(&nums).collect();
    }
    nums
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(blink(parse(input), 25).len())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(blink(parse(input), 75).len())
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
        assert_eq!(result, None);
    }
}
