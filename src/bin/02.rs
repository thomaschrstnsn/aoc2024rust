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

pub fn part_two(input: &str) -> Option<usize> {
    None
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
