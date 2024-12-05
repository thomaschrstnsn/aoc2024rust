use itertools::Itertools;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Input<'a>(Vec<&'a str>);

#[derive(Debug)]
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

impl Vec2<isize> {
    fn mul(&self, n: isize) -> Self {
        Vec2(self.0 * n, self.1 * n)
    }
}

type Direction = Vec2<isize>;
type Point = Vec2<usize>;

impl<'a> Input<'a> {
    fn from_str(input: &'a str) -> Option<Self> {
        let lines: Vec<&str> = input.lines().collect();

        let lengths = lines.iter().map(|l| l.len()).dedup().count();
        if lengths != 1 {
            return None;
        }

        Some(Self(lines))
    }

    fn columns(&self) -> usize {
        self.0.first().unwrap().len()
    }
    fn rows(&self) -> usize {
        self.0.len()
    }

    fn get(&self, p: &Point) -> Option<char> {
        self.0.get(p.0)?.chars().nth(p.1)
    }

    fn get_n_chars_in_direction<'b>(
        &'a self,
        n: usize,
        start: &'b Point,
        direction: &'b Direction,
    ) -> impl Iterator<Item = char> + use<'a, 'b> {
        (0..)
            .map(|i| start.add_signed(&direction.mul(i)))
            .take_while(Option::is_some)
            .map(Option::unwrap)
            .map(|p| self.get(&p))
            .take_while(Option::is_some)
            .map(Option::unwrap)
            .take(n)
    }

    fn is_same_as_in_direction<const N: usize>(
        &self,
        chars: [char; N],
        start: &Point,
        direction: &Direction,
    ) -> bool {
        self.get_n_chars_in_direction(chars.len(), start, direction)
            .eq(chars)
    }
}

fn x_marks_the_spot(point: &Point, input: &Input) -> usize {
    const DIRECTIONS: &[Direction] = &[
        Vec2(1, 0),
        Vec2(1, 1),
        Vec2(0, 1),
        Vec2(-1, 1),
        Vec2(-1, 0),
        Vec2(-1, -1),
        Vec2(0, -1),
        Vec2(1, -1),
    ];
    const TARGET: [char; 4] = ['X', 'M', 'A', 'S'];
    if input.get(point).unwrap() == 'X' {
        return DIRECTIONS
            .iter()
            .filter(|d| input.is_same_as_in_direction(TARGET, point, d))
            .count();
    }
    0
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = Input::from_str(input)?;

    let mut sum = 0;
    for x in 0..input.columns() {
        for y in 0..input.rows() {
            sum += x_marks_the_spot(&Vec2(x, y), &input);
        }
    }
    Some(sum)
}

fn mas_marks_the_x(point: &Point, input: &Input) -> usize {
    const TARGET: [char; 3] = ['M', 'A', 'S'];
    const TARGET_REV: [char; 3] = ['S', 'A', 'M'];

    let char_at_point = input.get(point).unwrap();
    if char_at_point == TARGET[0] || char_at_point == TARGET_REV[0] {
        let down_right = input.get_n_chars_in_direction(3, point, &Vec2(1, 1));
        let down_right2 = input.get_n_chars_in_direction(3, point, &Vec2(1, 1));

        if let Some(other_start) = point.add_signed(&Vec2(0, 2)) {
            let up_right = input.get_n_chars_in_direction(3, &other_start, &Vec2(1, -1));
            let up_right2 = input.get_n_chars_in_direction(3, &other_start, &Vec2(1, -1));

            if (down_right.eq(TARGET) || down_right2.eq(TARGET_REV))
                && (up_right.eq(TARGET) || up_right2.eq(TARGET_REV))
            {
                return 1;
            }
        }
    }
    0
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = Input::from_str(input)?;

    let mut sum = 0;
    for x in 0..input.columns() {
        for y in 0..input.rows() {
            sum += mas_marks_the_x(&Vec2(x, y), &input);
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
