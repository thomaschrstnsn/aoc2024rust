advent_of_code::solution!(14);

#[derive(Debug, PartialEq, Eq)]
struct Bounds {
    columns: u64,
    rows: u64,
}

impl Bounds {
    fn count_quadrants(&self, vecs: &[Vec2]) -> (usize, usize, usize, usize) {
        let mid_x = self.columns as i64 / 2;
        let mid_y = self.rows as i64 / 2;

        let q1_x = 0..mid_x;
        let q2_x = (mid_x + 1)..self.columns as i64;
        let q3_x = q1_x.clone();
        let q4_x = q2_x.clone();

        let q1_y = 0..mid_y;
        let q2_y = q1_y.clone();
        let q3_y = (mid_y + 1)..self.rows as i64;
        let q4_y = q3_y.clone();

        let mut q1_sum = 0;
        let mut q2_sum = 0;
        let mut q3_sum = 0;
        let mut q4_sum = 0;

        for vec in vecs {
            if q1_x.contains(&vec.0) && q1_y.contains(&vec.1) {
                q1_sum += 1;
            } else if q2_x.contains(&vec.0) && q2_y.contains(&vec.1) {
                q2_sum += 1;
            } else if q3_x.contains(&vec.0) && q3_y.contains(&vec.1) {
                q3_sum += 1;
            } else if q4_x.contains(&vec.0) && q4_y.contains(&vec.1) {
                q4_sum += 1;
            }
        }
        (q1_sum, q2_sum, q3_sum, q4_sum)
    }
}

#[derive(Debug, Hash, Ord, PartialEq, PartialOrd, Eq, Clone, Copy)]
struct Vec2(i64, i64);

impl Vec2 {
    fn add_capped(&self, other: &Vec2, bounds: &Bounds) -> Self {
        let mut r0 = self.0 + other.0;
        while r0.is_negative() {
            r0 += bounds.columns as i64;
        }
        let mut r1 = self.1 + other.1;
        while r1.is_negative() {
            r1 += bounds.rows as i64;
        }
        Self(r0 % bounds.columns as i64, r1 % bounds.rows as i64)
    }

    fn mult(&self, n: i64) -> Self {
        Self(self.0 * n, self.1 * n)
    }
}

impl Vec2 {
    /// Parses a Vec2 from a string with a prefix
    /// ```
    /// assert_eq!(Vec2::parse_with_prefix::<'p'>("p=0,4"), Ok(Vec2(0,4)));
    /// assert_eq!(Vec2::parse_with_prefix::<'v'>("v=-1,2"), Ok(Vec2(-1,2)));
    /// ```
    fn parse_with_prefix<const PREFIX: char>(input: &str) -> Result<Self, String> {
        let (prefix, vec) = input.split_once('=').ok_or("missing =")?;
        if prefix.len() != 1 {
            return Err(format!(
                "invalid prefix length (expected {}, got {})",
                1,
                prefix.len()
            ));
        }
        let actual_prefix = prefix.chars().next().unwrap();
        if actual_prefix != PREFIX {
            return Err(format!(
                "invalid prefix (expected {}, got {})",
                PREFIX, actual_prefix
            ));
        }

        let vec = vec.try_into()?;

        Ok(vec)
    }
}

impl TryFrom<&str> for Vec2 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (x, y) = value.split_once(",").ok_or("expected comma")?;

        let x = x.parse().map_err(|_| format!("failed to parse: '{}'", x))?;
        let y = y.parse().map_err(|_| format!("failed to parse: '{}'", y))?;

        Ok(Self(x, y))
    }
}

struct Robot {
    position: Vec2,
    direction: Vec2,
}

impl TryFrom<&str> for Robot {
    type Error = String;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (position, direction) = line.split_once(' ').ok_or("missing space")?;
        let position = Vec2::parse_with_prefix::<'p'>(position)?;
        let direction = Vec2::parse_with_prefix::<'v'>(direction)?;

        Ok(Self {
            position,
            direction,
        })
    }
}

struct Input(Vec<Robot>);

impl TryFrom<&str> for Input {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let robots = value
            .lines()
            .map(|line| line.try_into())
            .collect::<Result<_, _>>()?;
        Ok(Self(robots))
    }
}

fn part_one_with_bounds(input: &str, bounds: &Bounds) -> Option<usize> {
    let input: Input = input.try_into().expect("parses");
    let end_positions = input
        .0
        .iter()
        .map(|robot| {
            let direction = robot.direction.mult(100);
            robot.position.add_capped(&direction, bounds)
        })
        .collect::<Vec<_>>();

    let (q1, q2, q3, q4) = bounds.count_quadrants(&end_positions);

    Some(q1 * q2 * q3 * q4)
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_with_bounds(
        input,
        &Bounds {
            columns: 101,
            rows: 103,
        },
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds_count_quadrants() {
        let bounds = Bounds {
            columns: 11,
            rows: 7,
        };
        assert_eq!(bounds.count_quadrants(&[Vec2(0, 0)]), (1, 0, 0, 0));
        assert_eq!(bounds.count_quadrants(&[Vec2(6, 0)]), (0, 1, 0, 0));
        assert_eq!(bounds.count_quadrants(&[Vec2(0, 4)]), (0, 0, 1, 0));
        assert_eq!(bounds.count_quadrants(&[Vec2(6, 4)]), (0, 0, 0, 1));

        // on the borders
        assert_eq!(bounds.count_quadrants(&[Vec2(5, 0)]), (0, 0, 0, 0));
        assert_eq!(bounds.count_quadrants(&[Vec2(5, 3)]), (0, 0, 0, 0));
    }

    #[test]
    fn test_vec_add_capped_wrap_around_zero() {
        let o: Vec2 = Vec2(0, 0);
        let bounds = Bounds {
            columns: 100,
            rows: 50,
        };
        assert_eq!(o.add_capped(&Vec2(-1, 0), &bounds), Vec2(99, 0));
        assert_eq!(o.add_capped(&Vec2(-2, 0), &bounds), Vec2(98, 0));
        assert_eq!(o.add_capped(&Vec2(0, -1), &bounds), Vec2(0, 49));
        assert_eq!(o.add_capped(&Vec2(0, -2), &bounds), Vec2(0, 48));
        assert_eq!(o.add_capped(&Vec2(-2, -2), &bounds), Vec2(98, 48));
    }

    #[test]
    fn test_vec_add_capped_wrap_around_max() {
        let o: Vec2 = Vec2(99, 49);
        let bounds = Bounds {
            columns: 100,
            rows: 50,
        };
        assert_eq!(o.add_capped(&Vec2(0, 0), &bounds), Vec2(99, 49));
        assert_eq!(o.add_capped(&Vec2(1, 1), &bounds), Vec2(0, 0));
        assert_eq!(o.add_capped(&Vec2(2, 0), &bounds), Vec2(1, 49));
        assert_eq!(o.add_capped(&Vec2(3, 0), &bounds), Vec2(2, 49));
        assert_eq!(o.add_capped(&Vec2(0, 1), &bounds), Vec2(99, 0));
        assert_eq!(o.add_capped(&Vec2(0, 2), &bounds), Vec2(99, 1));
        assert_eq!(o.add_capped(&Vec2(0, 3), &bounds), Vec2(99, 2));
        assert_eq!(o.add_capped(&Vec2(3, 3), &bounds), Vec2(2, 2));
    }

    #[test]
    fn test_vec2_parse_with_prefix() {
        assert_eq!(Vec2::parse_with_prefix::<'p'>("p=0,4"), Ok(Vec2(0, 4)));
        assert_eq!(Vec2::parse_with_prefix::<'v'>("v=-1,2"), Ok(Vec2(-1, 2)));
    }

    #[test]
    fn test_part_one() {
        let result = part_one_with_bounds(
            &advent_of_code::template::read_file("examples", DAY),
            &Bounds {
                columns: 11,
                rows: 7,
            },
        );
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
