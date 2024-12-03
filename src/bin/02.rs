use itertools::Itertools;

advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|w| w.parse().expect("number"))
                .collect()
        })
        .collect()
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
}

type State = Result<Option<Direction>, ()>;

fn is_safe(row: &[i64]) -> bool {
    fn check(s: State, t: (&i64, &i64)) -> State {
        let delta = t.0 - t.1;
        if delta.abs() > 3 {
            return Err(());
        }
        let next_direction = match delta.signum() {
            0 => None,
            1 => Some(Direction::Up),
            -1 => Some(Direction::Down),
            _ => unreachable!(),
        };

        if next_direction.is_none() {
            return Err(());
        }
        let next_direction = next_direction.unwrap();
        match s {
            Ok(Some(direction)) => {
                if direction != next_direction {
                    Err(())
                } else {
                    s
                }
            }
            Ok(None) => Ok(Some(next_direction)),
            Err(_) => Err(()),
        }
    }
    row.iter().tuple_windows().fold(Ok(None), check).is_ok()
}

pub fn part_one(input: &str) -> Option<usize> {
    let parsed = parse(input);
    Some(parsed.iter().filter(|r| is_safe(r)).count())
}

fn is_safe_with_problem_dampener(r: &[i64]) -> bool {
    if is_safe(r) {
        return true;
    }

    for i in 0..r.len() {
        let (left, right) = r.split_at(i);
        let (_, skipped) = right.split_at(1);
        let mutation = [left, skipped].concat();
        if is_safe(&mutation) {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<usize> {
    let parsed = parse(input);
    Some(
        parsed
            .iter()
            .filter(|r| is_safe_with_problem_dampener(r))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe_1() {
        assert!(is_safe(&[7, 6, 4, 2, 1]));
    }

    #[test]
    fn test_is_safe_2() {
        assert!(!is_safe(&[1, 2, 7, 8, 9]));
    }

    #[test]
    fn test_is_safe_3() {
        assert!(!is_safe(&[9, 7, 6, 2, 1]));
    }

    #[test]
    fn test_is_safe_4() {
        assert!(!is_safe(&[1, 3, 2, 4, 5]));
    }

    #[test]
    fn test_is_safe_5() {
        assert!(!is_safe(&[8, 6, 4, 4, 1]));
    }

    #[test]
    fn test_is_safe_6() {
        assert!(is_safe(&[1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_is_safe_with_problem_dampener_1() {
        assert!(is_safe_with_problem_dampener(&[7, 6, 4, 2, 1]));
    }

    #[test]
    fn test_is_safe_with_problem_dampener_2() {
        assert!(!is_safe_with_problem_dampener(&[1, 2, 7, 8, 9]));
    }

    #[test]
    fn test_is_safe_with_problem_dampener_3() {
        assert!(!is_safe_with_problem_dampener(&[9, 7, 6, 2, 1]));
    }

    #[test]
    fn test_is_safe_with_problem_dampener_4() {
        assert!(is_safe_with_problem_dampener(&[1, 3, 2, 4, 5]));
    }

    #[test]
    fn test_is_safe_with_problem_dampener_5() {
        assert!(is_safe_with_problem_dampener(&[8, 6, 4, 4, 1]));
    }

    #[test]
    fn test_is_safe_with_problem_dampener_6() {
        assert!(is_safe_with_problem_dampener(&[1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_split_at() {
        let (left, right) = [1, 2, 3].split_at(0);
        assert_eq!(left, []);
        assert_eq!(right, [1, 2, 3]);
    }
}
