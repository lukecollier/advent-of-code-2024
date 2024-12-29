use std::time::Instant;

use itertools::Itertools;

advent_of_code::solution!(17);

#[derive(Debug)]
pub struct Computer {
    register_a: usize, // ???
    register_b: usize,
    register_c: usize,
    program: Vec<u8>,
    output: Vec<u8>,
    instruction_pointer: usize,
}

impl Computer {
    fn binary(&self) -> String {
        let Computer {
            register_a,
            register_b,
            register_c,
            program: _,
            output,
            instruction_pointer: _,
        } = self;
        let output_octals = output.iter().map(|num| format!("{num:b}")).join(",");
        format!("a({register_a:b}) b({register_b:b}) c({register_c:b}) output={output_octals}")
    }

    fn oct(&self) -> String {
        let Computer {
            register_a: _,
            register_b: _,
            register_c: _,
            program: _,
            output,
            instruction_pointer: _,
        } = self;
        let output_octals = output.iter().map(|num| format!("{num:o}")).join(",");
        format!("output={output_octals}")
    }

    fn combo(&self, operand: u8) -> usize {
        match operand {
            literal if literal <= 3 => literal as usize,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => panic!("Invalid program!!!"),
            _ => panic!("Unknown combo"),
        }
    }

    fn adv(&mut self, operand: u8) {
        let operand = self.combo(operand);
        let numerator = self.register_a;
        let denominator = 2_usize.pow(operand as u32);
        self.register_a = numerator / denominator;
        self.instruction_pointer += 2;
    }

    fn bxl(&mut self, operand: u8) {
        let xor = self.register_b ^ operand as usize;
        self.register_b = xor;
        self.instruction_pointer += 2;
    }

    fn bst(&mut self, operand: u8) {
        let operand = self.combo(operand) % 8;
        self.register_b = operand;
        self.instruction_pointer += 2;
    }

    fn jnz(&mut self, operand: u8) {
        if self.register_a == 0 {
            self.instruction_pointer += 2;
        } else {
            self.instruction_pointer = operand as usize;
        }
    }

    fn out(&mut self, operand: u8) {
        let operand = self.combo(operand) % 8;
        self.write(operand as u8);
        self.instruction_pointer += 2;
    }

    fn bxc(&mut self, _operand: u8) {
        self.register_b = self.register_b ^ self.register_c;
        self.instruction_pointer += 2;
    }

    fn bdv(&mut self, operand: u8) {
        let operand = self.combo(operand);
        let numerator = self.register_a;
        let denominator = 2_usize.pow(operand as u32);
        self.register_b = numerator / denominator;
        self.instruction_pointer += 2;
    }

    fn cdv(&mut self, operand: u8) {
        let operand = self.combo(operand);
        let numerator = self.register_a;
        let denominator = 2_usize.pow(operand as u32);
        self.register_c = numerator / denominator;
        self.instruction_pointer += 2;
    }

    fn write(&mut self, output: u8) {
        self.output.push(output);
    }

    fn execute(&mut self) -> String {
        while self.instruction_pointer < self.program.len() - 1 {
            let op = self.program[self.instruction_pointer];
            let operand = self.program[self.instruction_pointer + 1];
            match op {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => panic!("Op code ({}) not recognised", op),
            }
        }
        self.output.clone().into_iter().join(",")
    }

    fn parse(input: &str) -> Computer {
        let mut lines = input.lines();
        let register_a = lines
            .next()
            .expect("Register A not found")
            .strip_prefix("Register A: ")
            .expect("Could not strip prefix for Register A")
            .parse::<usize>()
            .expect("Could not parse Register A");
        let register_b = lines
            .next()
            .expect("Register B not found")
            .strip_prefix("Register B: ")
            .expect("Could not strip prefix for Register B")
            .parse::<usize>()
            .expect("Could not parse Register B");
        let register_c = lines
            .next()
            .expect("Register C not found")
            .strip_prefix("Register C: ")
            .expect("Could not strip prefix for Register C")
            .parse::<usize>()
            .expect("Could not parse Register B");
        assert!(lines.next().is_some_and(|a| a.is_empty()));
        let program = lines
            .next()
            .expect("Program not found")
            .strip_prefix("Program: ")
            .unwrap()
            .split(",")
            .map(|num| {
                dbg!(num);
                num.parse::<u8>().expect("Could not program")
            })
            .collect_vec();
        Computer {
            register_a,
            register_b,
            register_c,
            output: Vec::with_capacity(program.len()),
            program,
            instruction_pointer: 0,
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::parse(input);
    Some(computer.execute())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut computer = Computer::parse(input);
    let original_program = computer.program.clone();
    let original_program_string = computer.program.iter().join(",");
    let mut target = 1;
    let low = 0o1 * 0o10_usize.pow((original_program.len() - target) as u32);
    let mut register_a = low;
    let mut stack: Vec<usize> = Vec::new();
    loop {
        computer.program = original_program.clone();
        computer.output.clear();
        computer.instruction_pointer = 0;
        computer.register_a = register_a;
        let output = computer.execute();
        println!("input={register_a}({register_a:o}) output={output}");
        assert_eq!(output.len(), original_program_string.len());
        if output.len() == original_program_string.len() && output == original_program_string {
            return Some(register_a);
        }
        dbg!(original_program.len(), target);
        let start = original_program.len() - target;
        let end = original_program.len();
        if original_program[start..end] == computer.output[start..end] {
            stack.push(computer.register_a);
            target += 1;
        }
        // todo: backtracking, currently we have a problem. If we go down the wrong path we will
        // never find the right value.
        // So what we need to do is if we cycle through every octet at that position and we don't
        // find we need to pop from the stack and set our register back to this value
        // let high = 0o1 * 0o10_usize.pow((original_program.len() - target + 1) as u32);
        // println!("high={high}({high:o})");
        register_a += 0o1 * 0o10_usize.pow((original_program.len() - target) as u32);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
        let mut computer = Computer {
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            program: vec![0, 1, 5, 4, 3, 0],
            output: vec![],
            instruction_pointer: 0,
        };
        let result = computer.execute();
        assert_eq!(computer.register_a, 0);
        assert_eq!(result, String::from("4,2,5,6,7,7,7,7,3,1,0"));
    }

    #[test]
    fn test_bst() {
        let mut computer = Computer {
            register_a: 0,
            register_b: 0,
            register_c: 9,
            program: vec![2, 6],
            output: vec![],
            instruction_pointer: 0,
        };
        let result = computer.execute();
        assert_eq!(computer.register_b, 1);
        assert_eq!(result, String::from(""));
    }

    #[test]
    fn test_bxc() {
        let mut computer = Computer {
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            program: vec![4, 0],
            output: vec![],
            instruction_pointer: 0,
        };
        let result = computer.execute();
        assert_eq!(computer.register_b, 44354);
        assert_eq!(result, String::from(""));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("4,6,3,5,6,3,5,2,1,0")));
    }

    #[test]
    fn test_part_two() {
        let input = &r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;
        let result = part_two(input);
        assert_eq!(result, Some(117440));
    }
}
