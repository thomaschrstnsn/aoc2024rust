use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug, Hash, Ord, PartialEq, PartialOrd, Eq, Clone, Copy)]
struct Vec2<T>(T, T);

impl Vec2<usize> {
    fn add_signed(&self, other: &Vec2<isize>) -> Option<Self> {
        let (r0, o0) = self.0.overflowing_add_signed(other.0);
        let (r1, o1) = self.1.overflowing_add_signed(other.1);
        if o0 || o1 {
            return None;
        }
        Some(Self(r0, r1))
    }

    fn add_direction(&self, d: Direction) -> Option<Self> {
        self.add_signed(&d.into())
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for Vec2<isize> {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Vec2(0, -1),
            Direction::Down => Vec2(0, 1),
            Direction::Left => Vec2(-1, 0),
            Direction::Right => Vec2(1, 0),
        }
    }
}

type Position = Vec2<usize>;

struct Input(Vec<Vec<Option<u8>>>);

impl Input {
    fn parse_line(line: &str) -> Vec<Option<u8>> {
        line.chars()
            .map(|c| c.to_digit(10).map(|n| n as u8))
            .collect()
    }

    fn get(&self, p: &Position) -> Option<u8> {
        self.0.get(p.1)?.get(p.0).copied()?
    }

    fn rows(&self) -> usize {
        self.0.len()
    }

    fn columns(&self) -> usize {
        self.0.first().expect("not empty").len()
    }

    fn trailhead_score_by_destination(&self, p: &Position) -> usize {
        let mut visited_peaks: HashSet<Position> = HashSet::new();
        self.trailhead_helper(p, 0, &mut visited_peaks);
        visited_peaks.len()
    }

    fn trailhead_score_by_path(&self, p: &Position) -> Option<usize> {
        let mut visited_peaks: HashSet<Position> = HashSet::new();
        self.trailhead_helper(p, 0, &mut visited_peaks)
    }

    fn trailhead_helper(
        &self,
        p: &Position,
        level: u8,
        visited_peaks: &mut HashSet<Position>,
    ) -> Option<usize> {
        let value = self.get(p)?;
        if value != level {
            return None;
        }

        if value == 9 {
            visited_peaks.insert(*p);
            return Some(1);
        }

        let next_level = level + 1;
        let result = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .filter_map(|d| p.add_direction(d))
        .filter_map(|next_p| self.trailhead_helper(&next_p, next_level, visited_peaks))
        .sum();
        Some(result)
    }
}

impl TryFrom<&str> for Input {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines: Vec<Vec<Option<u8>>> = value.lines().map(Input::parse_line).collect();
        let lengths = lines.iter().map(|l| l.len()).dedup().count();
        if lengths != 1 {
            return Err("lines are not equal length".to_string());
        }
        Ok(Self(lines))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input: Input = input.try_into().ok()?;

    let mut sum = 0;
    for row in 0..input.rows() {
        for column in 0..input.columns() {
            let p = Vec2(column, row);
            let score = input.trailhead_score_by_destination(&p);
            if score > 0 {
                sum += score;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input: Input = input.try_into().ok()?;

    let mut sum = 0;
    for row in 0..input.rows() {
        for column in 0..input.columns() {
            let p = Vec2(column, row);
            if let Some(score) = input.trailhead_score_by_path(&p) {
                sum += score;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: Input = r"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"
            .try_into()
            .expect("parses");

        let trailhead_score = input.trailhead_score_by_destination(&Vec2(3, 0));

        assert_eq!(trailhead_score, 2);
    }

    #[test]
    fn test_example_2() {
        let input: Input = r"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."
            .try_into()
            .expect("parses");

        let trailhead_score = input.trailhead_score_by_destination(&Vec2(3, 0));

        assert_eq!(trailhead_score, 4);
    }

    #[test]
    fn test_vec2_underflow() {
        let o: Position = Vec2(0, 0);
        assert_eq!(None, o.add_signed(&Vec2(-1, 0)));
        assert_eq!(None, o.add_signed(&Vec2(0, -1)));
        assert_eq!(Some(o), o.add_signed(&Vec2(0, 0)));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
