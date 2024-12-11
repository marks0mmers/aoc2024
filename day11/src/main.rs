use std::{collections::HashMap, iter::repeat};

use utils::AdventOfCode;

struct State {
    stones: HashMap<usize, usize>,
}

impl State {
    fn new(input: &str) -> Self {
        Self {
            stones: input
                .trim()
                .split_whitespace()
                .filter_map(|num| num.parse().ok().map(|num| (num, 1)))
                .collect(),
        }
    }

    fn blink(&mut self) {
        let mut new_stones = HashMap::new();
        for (stone, count) in self.stones.iter() {
            let stone_str = stone.to_string();

            let stones = match *stone {
                0 => vec![1],
                _ if stone_str.len() % 2 == 0 => {
                    let (left, right) = stone_str.split_at(stone_str.len() / 2);
                    match utils::parse_tuple(left, right) {
                        Ok((left, right)) => {
                            vec![left, right]
                        }
                        _ => vec![],
                    }
                }
                stone => vec![stone * 2024],
            };

            for stone in stones {
                new_stones
                    .entry(stone)
                    .and_modify(|num| *num += *count)
                    .or_insert(*count);
            }
        }
        self.stones = new_stones;
    }
}

struct Day11;

impl AdventOfCode for Day11 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let mut state = State::new(input);
        repeat(()).take(25).for_each(|_| state.blink());
        return state.stones.values().sum();
    }

    fn part2(input: &str) -> Self::Output {
        let mut state = State::new(input);
        repeat(()).take(75).for_each(|_| state.blink());
        return state.stones.values().sum();
    }
}

fn main() {
    Day11::run(11);
}

#[cfg(test)]
mod tests {
    use crate::Day11;
    use utils::AdventOfCode;

    const INPUT: &str = "125 17";

    #[test]
    fn day11_part1() {
        let res = Day11::part1(INPUT);
        assert_eq!(res, 55312);
    }

    #[test]
    fn day11_part2() {
        let res = Day11::part2(INPUT);
        assert_eq!(res, 65601038650482);
    }
}
