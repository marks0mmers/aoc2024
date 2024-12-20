use std::collections::HashSet;

use dashmap::DashMap;
use pathfinding::prelude::{astar, bfs_reach};
use rayon::prelude::*;
use utils::{AdventOfCode, Direction, Point};

struct Racetrack {
    walls: HashSet<Point>,
    start: Point,
    end: Point,
    size: Point,
}

impl Racetrack {
    fn new(input: &str) -> Self {
        let height = input.lines().count();
        let width = input.lines().nth(0).unwrap().len();
        let grid_iter = input.lines().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x, y).into(), c))
        });

        let (start, _) = grid_iter.clone().find(|(_, c)| *c == 'S').unwrap();
        let (end, _) = grid_iter.clone().find(|(_, c)| *c == 'E').unwrap();
        let walls = grid_iter
            .filter(|(_, c)| *c == '#')
            .map(|(pos, _)| pos)
            .collect();

        Self {
            walls,
            start,
            end,
            size: (width, height).into(),
        }
    }

    fn find_best_path(&self) -> Vec<Point> {
        let (path, _) = astar(
            &self.start,
            |&p| {
                Direction::all()
                    .iter()
                    .map(|dir| p + dir.offset())
                    .filter(|p| !self.walls.contains(p))
                    .map(|p| (p, 1))
                    .collect::<Vec<_>>()
            },
            |p| p.rect_dist(self.end),
            |&p| p == self.end,
        )
        .unwrap();

        return path;
    }

    fn in_bounds(&self, p: Point) -> bool {
        (0..self.size.x).contains(&p.x) && (0..self.size.y).contains(&p.y)
    }

    fn find_cheats(&self, allowed: isize, min_saved: usize) -> DashMap<usize, usize> {
        let cheats = DashMap::new();
        let path = self.find_best_path();

        path.par_iter().enumerate().for_each(|(i, point)| {
            let mut ends = Vec::new();
            for end in bfs_reach(*point, |&p| {
                Direction::all()
                    .iter()
                    .map(|dir| p + dir.offset())
                    .filter(|p| p.rect_dist(*point) <= allowed && self.in_bounds(*p))
                    .collect::<Vec<_>>()
            })
            .filter(|p| {
                path.iter()
                    .position(|path| path == p)
                    .is_some_and(|pos| pos > i)
            }) {
                if !ends.contains(&end) {
                    let j = path.iter().position(|pos| *pos == end).unwrap();
                    let new_len =
                        [&path[..i], &path[j..]].concat().len() + point.rect_dist(end) as usize;
                    let diff = path.len() - new_len;
                    if diff >= min_saved {
                        cheats
                            .entry(path.len() - new_len)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                    ends.push(end);
                }
            }
        });

        return cheats;
    }
}

struct Day20;

impl AdventOfCode for Day20 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let track = Racetrack::new(input);
        let min = if cfg!(test) { 10 } else { 100 };
        return track
            .find_cheats(2, min)
            .iter()
            .filter_map(|entry| match *entry.key() >= min {
                true => Some(*entry.value()),
                false => None,
            })
            .sum();
    }

    fn part2(input: &str) -> Self::Output {
        let track = Racetrack::new(input);
        let min = if cfg!(test) { 70 } else { 100 };
        return track
            .find_cheats(20, min)
            .iter()
            .filter_map(|entry| match *entry.key() >= min {
                true => Some(*entry.value()),
                false => None,
            })
            .sum();
    }
}

fn main() {
    Day20::run(20);
}

#[cfg(test)]
mod tests {
    use crate::Day20;
    use utils::AdventOfCode;

    const INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn day20_part1() {
        let res = Day20::part1(INPUT);
        assert_eq!(res, 10);
    }

    #[test]
    fn day20_part2() {
        let res = Day20::part2(INPUT);
        assert_eq!(res, 41);
    }
}
