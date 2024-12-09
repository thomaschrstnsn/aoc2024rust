use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(6);

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
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn as_vec2(&self) -> Vec2<isize> {
        match self {
            Self::Up => Vec2(0, -1),
            Self::Down => Vec2(0, 1),
            Self::Left => Vec2(-1, 0),
            Self::Right => Vec2(1, 0),
        }
    }
}

type Position = Vec2<usize>;

struct Input<'a>(Vec<&'a str>);

impl<'a> Input<'a> {
    fn new(input: &'a str) -> Option<Self> {
        let lines: Vec<_> = input.lines().collect();
        let lengths = lines.iter().map(|l| l.len()).dedup().count();
        if lengths != 1 {
            return None;
        }
        Some(Self(lines))
    }

    fn get(&self, p: &Position) -> Option<char> {
        self.0.get(p.1)?.chars().nth(p.0)
    }
}

#[derive(Debug)]
struct State {
    guard: Position,
    direction: Direction,
    visited: HashSet<Position>,
}

impl State {
    fn initial(input: &Input) -> Option<Self> {
        let y = input
            .0
            .iter()
            .enumerate()
            .filter_map(|(i, line)| if line.contains('^') { Some(i) } else { None })
            .next()?;

        let x = input.0[y].find('^')?;

        Some(State {
            guard: Vec2(x, y),
            direction: Direction::Up,
            visited: Default::default(),
        })
    }

    fn get_next_position(
        &mut self,
        input: &Input,
        injected_obstacle: &Option<Position>,
    ) -> Option<Position> {
        self.visited.insert(self.guard);
        let mut next = self.guard.add_signed(&self.direction.as_vec2())?;

        let next_char = input.get(&next);

        let next_is_injected_obstacle = injected_obstacle.filter(|io| next == *io);
        let next_is_obstacle = next_char == Some('#') || next_is_injected_obstacle.is_some();
        if next_is_obstacle {
            self.direction = self.direction.turn_right();
            next = self
                .guard
                .add_signed(&self.direction.as_vec2())
                .expect("should this happen?");
        }
        Some(next)
    }

    fn move_guard_one_iteration(&mut self, input: &Input, injected_obstacle: &Option<Position>) {
        if let Some(next) = self.get_next_position(input, injected_obstacle) {
            self.guard = next;
        }
    }

    fn is_loop_configuration(
        &mut self,
        input: &Input,
        injected_obstacle: &Option<Position>,
    ) -> bool {
        let mut visited_with_direction = HashSet::new();
        visited_with_direction.insert((self.guard, self.direction));
        loop {
            if self.is_out_of_bounds(input) {
                return false;
            }
            if let Some(next) = self.get_next_position(input, injected_obstacle) {
                if visited_with_direction.contains(&(next, self.direction)) {
                    return true;
                }
                visited_with_direction.insert((next, self.direction));
                self.guard = next;
            } else {
                return false;
            }
        }
    }

    fn is_out_of_bounds(&self, input: &Input) -> bool {
        input.get(&self.guard).is_none()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = Input::new(input)?;
    let mut state = State::initial(&input)?;
    while !state.is_out_of_bounds(&input) {
        state.move_guard_one_iteration(&input, &None);
    }

    Some(state.visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = Input::new(input)?;
    let mut state_out_of_bounds = State::initial(&input)?;
    while !state_out_of_bounds.is_out_of_bounds(&input) {
        state_out_of_bounds.move_guard_one_iteration(&input, &None);
    }

    let mut sum = 0;
    for injected_obstacle in state_out_of_bounds.visited {
        let mut state = State::initial(&input)?;
        if state.is_loop_configuration(&input, &Some(injected_obstacle)) {
            sum += 1;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
