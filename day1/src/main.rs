use std::collections::HashMap;

use iter_tools::Itertools;

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

fn part1(input: &str) -> usize {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    return left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum();
}

fn part2(input: &str) -> usize {
    let (left, right) = parse_input(input);

    let right_count = right.iter().fold(HashMap::new(), |mut map, val| {
        map.entry(*val).and_modify(|n| *n += 1).or_insert(1usize);
        map
    });

    return left
        .iter()
        .map(|num| num * right_count.get(num).unwrap_or(&0))
        .sum();
}

fn main() {
    let input = utils::read_input_file(1).expect("failed to open input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn day1_part1() {
        let res = part1(INPUT);
        assert_eq!(res, 11);
    }

    #[test]
    fn day1_part2() {
        let res = part2(INPUT);
        assert_eq!(res, 31);
    }
}
