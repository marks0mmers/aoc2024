use std::collections::{HashMap, HashSet};

use utils::{AdventOfCode, Direction, Vec2};

struct TrailMap {
    map: HashMap<Vec2, usize>,
}

impl TrailMap {
    fn new(input: &str) -> Self {
        Self {
            map: input
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, c)| {
                        c.to_digit(10).map(|num| ((x, y).into(), num as usize))
                    })
                })
                .collect(),
        }
    }

    fn surronding(&self, pos: Vec2) -> [Vec2; 4] {
        return [
            pos + Direction::North.get_offset(),
            pos + Direction::East.get_offset(),
            pos + Direction::South.get_offset(),
            pos + Direction::West.get_offset(),
        ];
    }

    fn score<'a>(&'a self, pos: &'a Vec2, height: &usize) -> HashSet<&'a Vec2> {
        match height {
            9 => HashSet::from([pos]),
            height => self
                .surronding(*pos)
                .iter()
                .filter_map(|pos| self.map.get_key_value(pos))
                .filter(|(_, new_height)| height + 1 == **new_height)
                .fold(HashSet::new(), |mut set, (pos, height)| {
                    set.extend(self.score(pos, height));
                    set
                }),
        }
    }

    fn rating(&self, pos: &Vec2, height: &usize) -> usize {
        match height {
            9 => 1,
            height => self
                .surronding(*pos)
                .iter()
                .filter_map(|pos| self.map.get_key_value(pos))
                .filter(|(_, new_height)| height + 1 == **new_height)
                .map(|(pos, height)| self.rating(pos, height))
                .sum(),
        }
    }
}

struct Day10;

impl AdventOfCode for Day10 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let trails = TrailMap::new(input);
        return trails
            .map
            .iter()
            .filter(|(_, height)| **height == 0)
            .map(|(pos, height)| trails.score(pos, height).len())
            .sum();
    }

    fn part2(input: &str) -> Self::Output {
        let trails = TrailMap::new(input);
        return trails
            .map
            .iter()
            .filter(|(_, height)| **height == 0)
            .map(|(pos, height)| trails.rating(pos, height))
            .sum();
    }
}

fn main() {
    Day10::run(10);
}

#[cfg(test)]
mod tests {
    use crate::Day10;
    use utils::AdventOfCode;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn day10_part1() {
        let res = Day10::part1(INPUT);
        assert_eq!(res, 36);
    }

    #[test]
    fn day10_part2() {
        let res = Day10::part2(INPUT);
        assert_eq!(res, 81);
    }
}
