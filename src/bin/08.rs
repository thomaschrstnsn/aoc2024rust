use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(8);

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

    fn subtract(&self, other: &Vec2<usize>) -> Direction {
        let dx = self.0 as isize - other.0 as isize;
        let dy = self.1 as isize - other.1 as isize;
        Vec2(dx, dy)
    }
}

type Direction = Vec2<isize>;

impl Vec2<isize> {
    fn mult(&self, m: isize) -> Self {
        Self(self.0 * m, self.1 * m)
    }
}

type Position = Vec2<usize>;

#[derive(Debug)]
struct Antenna {
    position: Position,
    typ: char,
}

#[derive(Debug)]
struct Input {
    antennas: Vec<Antenna>,
    dimensions: Vec2<usize>,
}

impl TryFrom<&str> for Input {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let antennas = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c.is_alphabetic() || c.is_numeric() {
                        Some(Antenna {
                            position: Vec2(x, y),
                            typ: c,
                        })
                    } else {
                        None
                    }
                })
            })
            .collect();
        let column_lengths: Vec<_> = value.lines().map(|l| l.len()).dedup().collect();
        if column_lengths.len() != 1 {
            return Err("lines are not equal length".to_string());
        }

        let dim_x = column_lengths[0];
        let dim_y = value.lines().count();

        Ok(Self {
            antennas,
            dimensions: Vec2(dim_x, dim_y),
        })
    }
}

fn antinodes_for_type<'a>(antennas: &'a [&'a Antenna]) -> impl Iterator<Item = Position> + use<'a> {
    antennas
        .iter()
        .tuple_combinations()
        .flat_map(|(a, b)| {
            let delta_ab = a.position.subtract(&b.position);

            let delta_ba = b.position.subtract(&a.position);
            let antinode_a = b.position.add_signed(&delta_ba);
            let antinode_b = a.position.add_signed(&delta_ab);

            [antinode_a, antinode_b].into_iter()
        })
        .flatten()
}

impl Input {
    fn group_by_type(&self) -> HashMap<char, Vec<&Antenna>> {
        let mut result = HashMap::new();
        for (typ, vals) in &self.antennas.iter().chunk_by(|a| a.typ) {
            let entry = result.entry(typ).or_insert(vec![]);
            for val in vals {
                entry.push(val);
            }
        }
        result
    }
    fn antinodes(&self) -> HashSet<Position> {
        let mut antinodes = HashSet::new();
        let group_by_type = self.group_by_type();
        for (_, chunk) in group_by_type {
            for anti in antinodes_for_type(&chunk) {
                if anti.0 < self.dimensions.0 && anti.1 < self.dimensions.1 {
                    antinodes.insert(anti);
                }
            }
        }
        antinodes

        // self.antennas
        //     .iter()
        //     .chunk_by(|a| a.typ)
        //     .into_iter()
        //     .flat_map(|(_, g)| {
        //         antinodes_for_type(&g.collect::<Vec<&Antenna>>()).collect::<Vec<_>>()
        //     })
        //     .filter(|Vec2(x, y)| *x <= self.dimensions.0 && *y <= self.dimensions.1)
        //     .collect()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input: Input = input.try_into().ok()?;
    let antinodes = input.antinodes();
    antinodes.len().into()
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
