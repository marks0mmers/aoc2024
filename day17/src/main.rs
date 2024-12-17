use utils::AdventOfCode;

enum Instruction {
    Adv(usize),
    Bxl(usize),
    Bst(usize),
    Jnz(usize),
    Bxc,
    Out(usize),
    Bdv(usize),
    Cdv(usize),
}

impl Instruction {
    fn new(num: usize, operand: usize) -> Self {
        match num {
            0 => Self::Adv(operand),
            1 => Self::Bxl(operand),
            2 => Self::Bst(operand),
            3 => Self::Jnz(operand),
            4 => Self::Bxc,
            5 => Self::Out(operand),
            6 => Self::Bdv(operand),
            7 => Self::Cdv(operand),
            _ => panic!("invalid instruction"),
        }
    }
}

struct CPU {
    a: usize,
    b: usize,
    c: usize,
    pc: usize,
    instructions: Vec<usize>,
}

impl CPU {
    fn new(input: &str) -> Option<Self> {
        let (registers, program) = input.split_once("\n\n")?;

        let mut registers = registers
            .lines()
            .filter_map(|line| line.split_once(": "))
            .filter_map(|(_, num)| num.parse().ok());

        let a = registers.next()?;
        let b = registers.next()?;
        let c = registers.next()?;

        let instructions = program
            .split(": ")
            .nth(1)?
            .split(",")
            .filter_map(|num| num.trim().parse().ok())
            .collect();

        Some(Self {
            a,
            b,
            c,
            pc: 0,
            instructions,
        })
    }

    fn seeded(a: usize, instructions: Vec<usize>) -> Self {
        Self {
            a,
            b: 0,
            c: 0,
            pc: 0,
            instructions,
        }
    }

    fn combo(&self, operand: usize) -> usize {
        match operand {
            0..4 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid operand"),
        }
    }

    fn run(&mut self) -> Vec<usize> {
        let mut output = Vec::new();
        while self.pc < self.instructions.len() {
            let ins = Instruction::new(self.instructions[self.pc], self.instructions[self.pc + 1]);

            match ins {
                Instruction::Adv(op) => self.a /= 2usize.pow(self.combo(op) as u32),
                Instruction::Bxl(op) => self.b ^= op,
                Instruction::Bst(op) => self.b = self.combo(op) % 8,
                Instruction::Jnz(op) if self.a > 0 => self.pc = op,
                Instruction::Jnz(_) => self.pc += 2,
                Instruction::Bxc => self.b ^= self.c,
                Instruction::Out(op) => output.push(self.combo(op) % 8),
                Instruction::Bdv(op) => self.b = self.a / 2usize.pow(self.combo(op) as u32),
                Instruction::Cdv(op) => self.c = self.a / 2usize.pow(self.combo(op) as u32),
            }

            if !matches!(ins, Instruction::Jnz(_)) {
                self.pc += 2;
            }
        }
        return output;
    }

    fn find_repeat_program(&mut self) -> String {
        let mut seed = 0;
        for (i, _) in self.instructions.iter().enumerate().rev() {
            seed <<= 3;
            let mut res = CPU::seeded(seed, self.instructions.clone()).run();

            while res != &self.instructions[i..] {
                seed += 1;
                res = CPU::seeded(seed, self.instructions.clone()).run();
            }
        }
        return seed.to_string();
    }
}

struct Day17;

impl AdventOfCode for Day17 {
    type Output = String;

    fn part1(input: &str) -> Self::Output {
        return CPU::new(input).map_or(String::new(), |mut cpu| {
            cpu.run()
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<_>>()
                .join(",")
        });
    }

    fn part2(input: &str) -> Self::Output {
        return CPU::new(input).map_or(String::new(), |mut cpu| cpu.find_repeat_program());
    }
}

fn main() {
    Day17::run(17);
}

#[cfg(test)]
mod tests {
    use crate::Day17;
    use utils::AdventOfCode;

    const INPUT: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn day17_part1() {
        let res = Day17::part1(INPUT);
        assert_eq!(res, "5,7,3,0");
    }

    #[test]
    fn day17_part2() {
        let res = Day17::part2(INPUT);
        assert_eq!(res, "117440");
    }
}
