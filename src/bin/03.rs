advent_of_code::solution!(3);

type Multiplication = (u32, u32);

fn result(mults: &[Multiplication]) -> u32 {
    mults.iter().map(|(x, y)| (x * y)).sum()
}

fn parse_single(input: &str) -> (Option<Multiplication>, &str) {
    if let Some((_, start)) = input.split_once("mul(") {
        let range = if let Some((upto_next, _)) = start.split_once("mul(") {
            upto_next
        } else {
            start
        };
        let mult = find_one(range);

        return (mult, start);
    } else {
        return (None, "");
    }

    fn find_one(input: &str) -> Option<Multiplication> {
        let (potential_number1, rest) = input.split_once(",")?;
        let number1 = potential_number1.parse().ok()?;

        let (potential_number2, _) = rest.split_once(")")?;
        let number2 = potential_number2.parse().ok()?;

        Some((number1, number2))
    }
}

fn parse(input: &str) -> Vec<Multiplication> {
    let mut mults = Vec::new();
    let mut cur = input;
    while !cur.is_empty() {
        let (mult, rest) = parse_single(cur);
        if let Some(mult) = mult {
            mults.push(mult);
        }
        cur = rest;
    }

    mults
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = parse(input);
    Some(result(&parsed))
}

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Multiplication(Multiplication),
}

fn result_for_instructions(instructions: &[Instruction]) -> u32 {
    let mut doing = true;
    let mut sum_of_products = 0;
    for ins in instructions {
        match ins {
            Instruction::Do => doing = true,
            Instruction::Dont => doing = false,
            Instruction::Multiplication((x, y)) => {
                if doing {
                    sum_of_products += x * y
                }
            }
        }
    }

    sum_of_products
}

fn parse_single_instruction(input: &str) -> (Option<Instruction>, &str) {
    if let Some((before, start)) = input.split_once("(") {
        let range = if let Some((upto_next, _)) = start.split_once("(") {
            upto_next
        } else {
            start
        };

        if before.ends_with("mul") {
            let mult = parse_mult(range);
            return (mult.map(Instruction::Multiplication), start);
        } else if before.ends_with("do") && range.starts_with(")") {
            return (Some(Instruction::Do), start);
        } else if before.ends_with("don't") && range.starts_with(")") {
            return (Some(Instruction::Dont), start);
        };
        return (None, start);
    } else {
        return (None, "");
    }

    fn parse_mult(input: &str) -> Option<Multiplication> {
        let (potential_number1, rest) = input.split_once(",")?;
        let number1 = potential_number1.parse().ok()?;

        let (potential_number2, _) = rest.split_once(")")?;
        let number2 = potential_number2.parse().ok()?;

        Some((number1, number2))
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut cur = input;
    while !cur.is_empty() {
        let (mult, rest) = parse_single_instruction(cur);
        if let Some(mult) = mult {
            instructions.push(mult);
        }
        cur = rest;
    }

    instructions
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed = parse_instructions(input);
    Some(result_for_instructions(&parsed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
