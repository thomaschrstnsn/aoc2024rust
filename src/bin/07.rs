advent_of_code::solution!(7);

struct Input(Vec<Equation>);

#[derive(Debug)]
struct Equation {
    result: u64,
    components: Vec<u64>,
}

impl TryFrom<&str> for Input {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let equations = value
            .lines()
            .map(|line| line.try_into())
            .collect::<Result<_, _>>()?;
        Ok(Self(equations))
    }
}

impl TryFrom<&str> for Equation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (result_str, rest) = value.split_once(':').ok_or("missing colon")?;

        let result = result_str
            .parse()
            .map_err(|_| format!("not a number: {}", result_str))?;

        let components = rest
            .split_whitespace()
            .map(|s| {
                s.parse()
                    .map_err(|_| format!("not a number: {}", s))
                    .unwrap()
            })
            .collect();

        Ok(Self { result, components })
    }
}

trait Concatable
where
    Self: Sized,
{
    fn concat(&self, other: &Self) -> Option<Self>;
}

impl Concatable for u64 {
    fn concat(&self, other: &Self) -> Option<Self> {
        let concat = format!("{}{}", self, other);
        concat.parse().ok()
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum OperatorSet {
    AddMultiply,
    AddMultiplyConcat,
}

#[derive(Debug)]
struct Solution {
    operators: Vec<Operator>,
}

impl Solution {
    fn init(input: &Equation) -> Self {
        let needed_ops = input.components.len() - 1;
        Self {
            operators: Vec::with_capacity(needed_ops),
        }
    }

    fn trace_solution(
        &mut self,
        equation: &Equation,
        intermediate_result: u64,
        index: usize,
        ops: OperatorSet,
    ) -> bool {
        if let Some(component) = equation.components.get(index) {
            let multiplied = component * intermediate_result;
            if multiplied <= equation.result {
                self.operators.push(Operator::Multiply);
                if self.trace_solution(equation, multiplied, index + 1, ops) {
                    return true;
                }
                self.operators.pop();
            }
            let added = component + intermediate_result;
            if added <= equation.result {
                self.operators.push(Operator::Add);
                if self.trace_solution(equation, added, index + 1, ops) {
                    return true;
                }
                self.operators.pop();
            }
            let concated = intermediate_result
                .concat(component)
                .expect("can be concated");
            if ops == OperatorSet::AddMultiplyConcat && concated <= equation.result {
                self.operators.push(Operator::Concat);
                if self.trace_solution(equation, concated, index + 1, ops) {
                    return true;
                }
                self.operators.pop();
            }
        } else {
            return intermediate_result == equation.result;
        }

        false
    }

    fn find(equation: &Equation, ops: OperatorSet) -> Option<Solution> {
        let mut solution = Self::init(equation);

        let first = equation.components[0];
        if solution.trace_solution(equation, first, 1, ops) {
            return Some(solution);
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input: Input = input.try_into().expect("can be parsed");

    let mut sum = 0;
    for equation in input.0 {
        if Solution::find(&equation, OperatorSet::AddMultiply).is_some() {
            sum += equation.result;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input: Input = input.try_into().expect("can be parsed");

    let mut sum = 0;
    for equation in input.0 {
        if Solution::find(&equation, OperatorSet::AddMultiplyConcat).is_some() {
            sum += equation.result;
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
