use std::{fmt::Display, iter};

use itertools::Itertools;

advent_of_code::solution!(15);

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

#[derive(Copy, Debug, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Robot,
    Wall,
    Box,
}

struct Input {
    tiles: Vec<Vec<Tile>>,
    moves: Vec<Direction>,
}

impl Input {
    fn parse_tile_line(line: &str) -> Result<Vec<Tile>, String> {
        line.chars()
            .map(|c| match c {
                '.' => Ok(Tile::Empty),
                '@' => Ok(Tile::Robot),
                '#' => Ok(Tile::Wall),
                'O' => Ok(Tile::Box),
                _ => Err(format!("invalid tile character: {}", c)),
            })
            .collect()
    }

    fn parse_tiles<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Vec<Vec<Tile>>, String> {
        let lines: Vec<Vec<Tile>> = lines
            .map(Input::parse_tile_line)
            .collect::<Result<Vec<_>, _>>()?;
        let lengths = lines.iter().map(|l| l.len()).dedup().count();
        if lengths != 1 {
            return Err("lines are not equal length".to_string());
        }
        Ok(lines)
    }

    fn parse_moves<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Vec<Direction>, String> {
        lines
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '^' => Ok(Direction::Up),
                    'v' => Ok(Direction::Down),
                    '<' => Ok(Direction::Left),
                    '>' => Ok(Direction::Right),
                    _ => Err(format!("invalid move character: {}", c)),
                })
            })
            .collect()
    }

    fn get(&self, p: &Position) -> Option<Tile> {
        self.tiles.get(p.1)?.get(p.0).copied()
    }

    fn set(&mut self, tile: Tile, p: &Position) {
        let row = &mut self.tiles[p.1];
        row[p.0] = tile;
    }

    fn rows(&self) -> usize {
        self.tiles.len()
    }

    fn columns(&self) -> usize {
        self.tiles.first().expect("not empty").len()
    }

    fn find_robot(&self) -> Option<Position> {
        for y in 0..self.rows() {
            for x in 0..self.columns() {
                let pos = Vec2(x, y);
                if let Some(Tile::Robot) = self.get(&pos) {
                    return Some(pos);
                }
            }
        }
        None
    }

    fn swap(&mut self, first: &Position, second: &Position) {
        let first_value = self.get(first).expect("should have value");
        let second_value = self.get(second).expect("should have value");

        self.set(second_value, first);
        self.set(first_value, second);
    }

    fn execute_move(&mut self, mov: Direction, robot: &mut Position) {
        let moved = iter::successors(Some(*robot), |p| p.add_direction(mov));

        let mut swap_queue = Vec::new();
        for pos in moved {
            if let Some(tile) = self.get(&pos) {
                if tile == Tile::Wall {
                    break;
                }
                swap_queue.push(pos);
                if tile == Tile::Empty {
                    for (from, to) in swap_queue.into_iter().rev().tuple_windows() {
                        self.swap(&from, &to);
                    }
                    *robot = robot.add_direction(mov).expect("we can move");
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn execute(&mut self) {
        let mut robot = self.find_robot().expect("should have a robot");
        for mov in self.moves.clone() {
            self.execute_move(mov, &mut robot);
        }
    }

    fn box_gps_coords(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for y in 0..self.rows() {
            for x in 0..self.columns() {
                let pos = Vec2(x, y);
                if let Some(Tile::Box) = self.get(&pos) {
                    let gps = y * 100 + x;
                    result.push(gps);
                }
            }
        }
        result
    }
}

impl TryFrom<&str> for Input {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let tile_lines = input.lines().take_while(|l| !l.is_empty());
        let tiles = Input::parse_tiles(tile_lines)?;

        let move_lines = input.lines().skip_while(|l| !l.is_empty()).skip(1);
        let moves = Input::parse_moves(move_lines)?;

        Ok(Self { tiles, moves })
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                let c = match tile {
                    Tile::Empty => '.',
                    Tile::Robot => '@',
                    Tile::Wall => '#',
                    Tile::Box => 'O',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut input: Input = input.try_into().ok()?;
    input.execute();

    Some(input.box_gps_coords().into_iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_smaller() {
        let input = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
        let result = part_one(&input);
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
