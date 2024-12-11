use regex::Regex;
use utils::AdventOfCode;

struct Day3;

impl AdventOfCode for Day3 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let mut total = 0;

        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        for (_, [left, right]) in re.captures_iter(input).map(|c| c.extract()) {
            if let Ok((left, right)) = utils::parse_tuple::<usize>(left, right) {
                total += left * right;
            }
        }

        return total;
    }

    fn part2(input: &str) -> Self::Output {
        let mut total = 0;
        let mut should_mul = true;

        let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

        for c in re.captures_iter(input) {
            match c.get(0) {
                Some(m) if m.as_str() == "do()" => should_mul = true,
                Some(m) if m.as_str() == "don't()" => should_mul = false,
                Some(_) if should_mul => {
                    let left = c.get(1).and_then(|m| m.as_str().parse::<usize>().ok());
                    let right = c.get(2).and_then(|m| m.as_str().parse::<usize>().ok());

                    if let (Some(left), Some(right)) = (left, right) {
                        total += left * right;
                    }
                }
                _ => {}
            }
        }

        return total;
    }
}

fn main() {
    Day3::run(3);
}

#[cfg(test)]
mod tests {
    use crate::Day3;
    use utils::AdventOfCode;

    const INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn day3_part1() {
        let res = Day3::part1(INPUT);
        assert_eq!(res, 161);
    }

    #[test]
    fn day3_part2() {
        let res = Day3::part2(INPUT);
        assert_eq!(res, 48);
    }
}
