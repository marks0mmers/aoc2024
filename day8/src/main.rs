use std::collections::{HashMap, HashSet};

use utils::{AdventOfCode, Point};

struct Map {
    size: Point,
    antennas: HashMap<char, Vec<Point>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let size = (input.lines().nth(0).unwrap().len(), input.lines().count()).into();

        let mut antennas = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
                antennas
                    .entry(c)
                    .and_modify(|vecs: &mut Vec<Point>| vecs.push((x, y).into()))
                    .or_insert(vec![(x, y).into()]);
            }
        }

        Self { size, antennas }
    }

    fn antinodes(&self, extend: bool) -> HashSet<Point> {
        let mut set = HashSet::new();

        for antennas in self.antennas.values().filter(|a| a.len() > 1) {
            for (left, right) in antennas
                .iter()
                .enumerate()
                .flat_map(|(i, a)| antennas[i + 1..].iter().map(move |b| (a, b)))
            {
                let diff = *right - *left;
                let mut potential = Vec::new();
                if !extend {
                    let left_anti = *left - diff;
                    let right_anti = *right + diff;
                    potential.extend_from_slice(&[left_anti, right_anti]);
                } else {
                    let mut curr = *left;
                    while (0..self.size.x).contains(&curr.x) && (0..self.size.y).contains(&curr.y) {
                        potential.push(curr);
                        curr = curr - diff;
                    }
                    curr = *right;
                    while (0..self.size.x).contains(&curr.x) && (0..self.size.y).contains(&curr.y) {
                        potential.push(curr);
                        curr = curr + diff;
                    }
                }

                for pot in potential {
                    if (0..self.size.x).contains(&pot.x) && (0..self.size.y).contains(&pot.y) {
                        set.insert(pot);
                    }
                }
            }
        }

        return set;
    }
}

struct Day8;

impl AdventOfCode for Day8 {
    type Output = usize;
    fn part1(input: &str) -> Self::Output {
        let map = Map::new(input);
        return map.antinodes(false).len();
    }

    fn part2(input: &str) -> Self::Output {
        let map = Map::new(input);
        return map.antinodes(true).len();
    }
}

fn main() {
    Day8::run(8);
}

#[cfg(test)]
mod tests {
    use crate::Day8;
    use utils::AdventOfCode;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn day8_part1() {
        let res = Day8::part1(INPUT);
        assert_eq!(res, 14);
    }

    #[test]
    fn day8_part2() {
        let res = Day8::part2(INPUT);
        assert_eq!(res, 34);
    }
}
