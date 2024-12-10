use utils::AdventOfCode;

struct Equation {
    test: usize,
    numbers: Vec<usize>,
}

enum Op {
    Add,
    Mul,
    Concat,
}

impl Op {
    fn exec(&self, left: usize, right: &usize) -> usize {
        match self {
            Op::Add => left + right,
            Op::Mul => left * right,
            Op::Concat => format!("{left}{right}").parse().unwrap(),
        }
    }
}

impl Equation {
    fn new(input: &str) -> Self {
        let (test, numbers) = input.split_once(": ").expect("invalid line");

        let test = test.parse().unwrap();

        let numbers = numbers
            .split(" ")
            .filter_map(|num| num.parse().ok())
            .collect();

        Self { test, numbers }
    }

    fn is_valid(&self, ops: &[Op]) -> bool {
        fn check(test: usize, ops: &[Op], prev: usize, op: &Op, rest: &[usize]) -> bool {
            match rest {
                [next] => op.exec(prev, next) == test,
                [next, rest @ ..] => ops
                    .iter()
                    .any(|next_op| check(test, ops, op.exec(prev, next), next_op, rest)),
                [] => panic!("invalid"),
            }
        }

        return ops
            .iter()
            .any(|op| check(self.test, ops, 0, op, &self.numbers));
    }
}

struct Day7;

impl AdventOfCode for Day7 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        return input
            .lines()
            .map(|line| Equation::new(line))
            .filter(|eq| eq.is_valid(&[Op::Add, Op::Mul]))
            .map(|eq| eq.test)
            .sum();
    }

    fn part2(input: &str) -> Self::Output {
        return input
            .lines()
            .map(|line| Equation::new(line))
            .filter(|eq| eq.is_valid(&[Op::Add, Op::Mul, Op::Concat]))
            .map(|eq| eq.test)
            .sum();
    }
}

fn main() {
    Day7::run(7);
}

#[cfg(test)]
mod tests {
    use crate::Day7;
    use utils::AdventOfCode;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn day7_part1() {
        let res = Day7::part1(INPUT);
        assert_eq!(res, 3749);
    }

    #[test]
    fn day7_part2() {
        let res = Day7::part2(INPUT);
        assert_eq!(res, 11387);
    }
}
