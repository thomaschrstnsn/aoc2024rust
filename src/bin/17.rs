use itertools::Itertools;

advent_of_code::solution!(17);

#[derive(Debug, Eq, PartialEq)]
enum State {
    Running,
    Halted,
}

#[derive(Debug, Eq, PartialEq)]
struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    ip: usize,
    instructions: Vec<u8>,
    state: State,
}

impl TryFrom<&str> for Computer {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut lines = input.lines();

        let reg_a = lines.next().ok_or("needed line for reg a")?;
        let reg_a = Computer::parse_register::<'A'>(reg_a)?;

        let reg_b = lines.next().ok_or("needed line for reg b")?;
        let reg_b = Computer::parse_register::<'B'>(reg_b)?;

        let reg_c = lines.next().ok_or("needed line for reg c")?;
        let reg_c = Computer::parse_register::<'C'>(reg_c)?;

        let blank = lines.next();
        if blank.is_none() || !blank.unwrap().is_empty() {
            return Err("needed blank line".to_string());
        }

        if let Some(program_line) = lines.next() {
            let (program, instructions) = program_line
                .split_once(':')
                .ok_or(format!("needed a program line with colon"))?;

            if program != "Program" {
                return Err("needed program line".to_string());
            }

            let instructions = instructions
                .trim()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();

            Ok(Computer::new(instructions, reg_a, reg_b, reg_c))
        } else {
            Err("needed program line".to_string())
        }
    }
}

impl Computer {
    fn new(instructions: Vec<u8>, a: usize, b: usize, c: usize) -> Self {
        Self {
            reg_a: a,
            reg_b: b,
            reg_c: c,
            ip: 0,
            instructions,
            state: State::Running,
        }
    }

    fn parse_register<const PREFIX: char>(line: &str) -> Result<usize, String> {
        let (reg, val) = line
            .split_once(':')
            .ok_or(format!("could not split on : for input '{}'", line))?;
        let expected = format!("Register {PREFIX}");
        if reg != expected {
            return Err(format!("Expected: {}, found: {}", expected, reg));
        }

        val.trim()
            .parse()
            .map_err(|_| format!("could not parse '{}' as string", val))
    }

    fn execute(&mut self) -> Vec<u8> {
        let mut outputs = Vec::new();
        while self.state == State::Running {
            if let Some(output) = self.step() {
                outputs.push(output);
            }
        }
        outputs
    }

    /// Execute a single instruction and returns optional output
    fn step(&mut self) -> Option<u8> {
        debug_assert_eq!(self.state, State::Running);

        if let (Some(opcode), Some(operand)) = (
            self.instructions.get(self.ip),
            self.instructions.get(self.ip + 1),
        ) {
            let mut result = None;
            self.ip = match opcode {
                0 => self.instruction_adv(*operand),
                1 => self.instruction_bxl(*operand),
                2 => self.instruction_bst(*operand),
                3 => self.instruction_jnz(*operand),
                4 => self.instruction_bxc(*operand),
                5 => {
                    let (next, out) = self.instruction_out(*operand);
                    result = Some(out);
                    next
                }
                6 => self.instruction_bdv(*operand),
                7 => self.instruction_cdv(*operand),
                _ => panic!("Invalid instruction: {}", opcode),
            };
            result
        } else {
            self.state = State::Halted;
            None
        }
    }

    fn combo_operand(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand as usize,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid combo operand: {}", operand),
        }
    }

    fn division(&self, operand: u8) -> usize {
        let combo = self.combo_operand(operand);
        let numerator = self.reg_a;
        let denominator = 2_usize.pow(combo as u32);
        numerator / denominator
    }

    fn instruction_adv(&mut self, operand: u8) -> usize {
        self.reg_a = self.division(operand);
        self.ip + 2
    }
    fn instruction_bdv(&mut self, operand: u8) -> usize {
        self.reg_b = self.division(operand);
        self.ip + 2
    }
    fn instruction_cdv(&mut self, operand: u8) -> usize {
        self.reg_c = self.division(operand);
        self.ip + 2
    }
    fn instruction_bxc(&mut self, _operand: u8) -> usize {
        self.reg_b ^= self.reg_c;
        self.ip + 2
    }
    fn instruction_bxl(&mut self, operand: u8) -> usize {
        self.reg_b ^= operand as usize;
        self.ip + 2
    }
    fn instruction_bst(&mut self, operand: u8) -> usize {
        self.reg_b = self.combo_operand(operand) % 8;
        self.ip + 2
    }
    fn instruction_out(&mut self, operand: u8) -> (usize, u8) {
        (self.ip + 2, (self.combo_operand(operand) % 8) as u8)
    }
    fn instruction_jnz(&mut self, operand: u8) -> usize {
        if self.reg_a == 0 || self.ip == operand as usize {
            self.ip + 2
        } else {
            operand as usize
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer: Computer = input.try_into().ok()?;
    let output = computer.execute();

    Some(output.into_iter().map(|n| n.to_string()).join(","))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut computer = Computer::new(vec![2, 6], 0, 0, 9);
        let output = computer.execute();
        assert!(output.is_empty());
        assert_eq!(computer.reg_b, 1);
    }

    #[test]
    fn test_example_2() {
        let mut computer = Computer::new(vec![5, 0, 5, 1, 5, 4], 10, 0, 0);
        let output = computer.execute();
        assert_eq!(&output, &[0, 1, 2]);
    }

    #[test]
    fn test_example_3() {
        let mut computer = Computer::new(vec![0, 1, 5, 4, 3, 0], 2024, 0, 0);
        let output = computer.execute();
        assert_eq!(&output, &[4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.reg_a, 0);
    }

    #[test]
    fn test_example_4() {
        let mut computer = Computer::new(vec![1, 7], 0, 29, 0);
        let output = computer.execute();
        assert!(output.is_empty());
        assert_eq!(computer.reg_b, 26);
    }

    #[test]
    fn test_example_5() {
        let mut computer = Computer::new(vec![4, 0], 0, 2024, 43690);
        let output = computer.execute();
        assert!(output.is_empty());
        assert_eq!(computer.reg_b, 44354);
    }

    #[test]
    fn test_parse() {
        let input: &str = &advent_of_code::template::read_file("examples", DAY);
        let result: Result<Computer, String> = input.try_into();
        assert_eq!(result, Ok(Computer::new(vec![0, 1, 5, 4, 3, 0], 729, 0, 0)));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
