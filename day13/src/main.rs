use std::usize;

use regex::Regex;
use utils::{AdventOfCode, Vec2};

struct Day13;

struct ClawMachine {
    a: Vec2,
    b: Vec2,
    p: Vec2,
}

impl ClawMachine {
    fn new(input: &str, p_offset: usize) -> Self {
        let re = Regex::new(
            "Button A: X\\+(\\d+), Y\\+(\\d+)\nButton B: X\\+(\\d+), Y\\+(\\d+)\nPrize: X=(\\d+), Y=(\\d+)",
        )
        .unwrap();

        if let Some((_, [ax, ay, bx, by, px, py])) = re.captures(input).map(|c| c.extract()) {
            let a = Vec2::new(ax.parse().unwrap(), ay.parse().unwrap());
            let b = Vec2::new(bx.parse().unwrap(), by.parse().unwrap());
            let p =
                Vec2::new(px.parse().unwrap(), py.parse().unwrap()) + (p_offset, p_offset).into();
            return Self { a, b, p };
        }
        panic!("invalid input: {input}")
    }

    fn tokens(&self) -> Option<usize> {
        let det = self.a.x * self.b.y - self.a.y * self.b.x;
        if det == 0 {
            return None;
        }

        let a = self.b.y * self.p.x - self.b.x * self.p.y;
        let b = self.a.x * self.p.y - self.a.y * self.p.x;

        if a % det != 0 || b % det != 0 {
            return None;
        }

        return Some((3 * a / det + b / det) as usize);
    }
}

impl AdventOfCode for Day13 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        return input
            .split("\n\n")
            .map(|input| ClawMachine::new(input, 0))
            .filter_map(|cm| cm.tokens())
            .sum();
    }

    fn part2(input: &str) -> Self::Output {
        return input
            .split("\n\n")
            .map(|input| ClawMachine::new(input, 10000000000000))
            .filter_map(|cm| cm.tokens())
            .sum();
    }
}

fn main() {
    Day13::run(13);
}

#[cfg(test)]
mod tests {
    use crate::Day13;
    use utils::AdventOfCode;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn day13_part1() {
        let res = Day13::part1(INPUT);
        assert_eq!(res, 480);
    }

    #[test]
    fn day13_part2() {
        let res = Day13::part2(INPUT);
        assert_eq!(res, 875318608908);
    }
}
