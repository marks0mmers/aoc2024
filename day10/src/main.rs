use core::panic;
use std::{
    array,
    collections::{HashMap, HashSet},
};

use utils::{Direction, Vec2};

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
        array::from_fn(|i| match i {
            0 => pos + Direction::North.get_offset(),
            1 => pos + Direction::East.get_offset(),
            2 => pos + Direction::South.get_offset(),
            3 => pos + Direction::West.get_offset(),
            _ => panic!("invalid"),
        })
    }

    fn score(&self, pos: Vec2, height: usize, finishing: &mut HashSet<Vec2>) {
        match height {
            9 => {
                finishing.insert(pos);
            }
            height => self
                .surronding(pos)
                .iter()
                .filter_map(|pos| self.map.get_key_value(pos))
                .filter(|(_, new_height)| height + 1 == **new_height)
                .for_each(|(pos, height)| self.score(*pos, *height, finishing)),
        };
    }

    fn rating(&self, pos: Vec2, height: usize) -> usize {
        match height {
            9 => 1,
            height => self
                .surronding(pos)
                .iter()
                .filter_map(|pos| self.map.get_key_value(pos))
                .filter(|(_, new_height)| height + 1 == **new_height)
                .map(|(pos, height)| self.rating(*pos, *height))
                .sum(),
        }
    }
}

fn part1(input: &str) -> usize {
    let trails = TrailMap::new(input);
    return trails
        .map
        .iter()
        .filter(|(_, height)| **height == 0)
        .map(|(pos, height)| {
            let mut finishing = HashSet::new();
            trails.score(*pos, *height, &mut finishing);
            finishing.len()
        })
        .sum();
}

fn part2(input: &str) -> usize {
    let trails = TrailMap::new(input);
    return trails
        .map
        .iter()
        .filter(|(_, height)| **height == 0)
        .map(|(pos, height)| trails.rating(*pos, *height))
        .sum();
}

fn main() {
    let input = utils::read_input_file(10).expect("failed to open input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

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
        let res = part1(INPUT);
        assert_eq!(res, 36);
    }

    #[test]
    fn day10_part2() {
        let res = part2(INPUT);
        assert_eq!(res, 81);
    }
}