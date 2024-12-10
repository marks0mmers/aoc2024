use std::collections::HashMap;

use iter_tools::Itertools;
use utils::AdventOfCode;

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    return input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|chars| chars.parse::<usize>().expect("invalid number"))
                .collect_tuple()
                .expect("failed to produce tuple")
        })
        .unzip();
}

struct Day1;

impl AdventOfCode for Day1 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let (mut left, mut right) = parse_input(input);

        left.sort();
        right.sort();

        return left
            .iter()
            .zip(right.iter())
            .map(|(left, right)| left.abs_diff(*right))
            .sum();
    }

    fn part2(input: &str) -> Self::Output {
        let (left, right) = parse_input(input);

        let right_count = right.iter().fold(HashMap::new(), |mut map, val| {
            map.entry(*val).and_modify(|n| *n += 1).or_insert(1);
            map
        });

        return left
            .iter()
            .map(|num| num * right_count.get(num).unwrap_or(&0))
            .sum();
    }
}

fn main() {
    Day1::run(1);
}

#[cfg(test)]
mod tests {
    use crate::Day1;
    use utils::AdventOfCode;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn day1_part1() {
        let res = Day1::part1(INPUT);
        assert_eq!(res, 11);
    }

    #[test]
    fn day1_part2() {
        let res = Day1::part2(INPUT);
        assert_eq!(res, 31);
    }
}
