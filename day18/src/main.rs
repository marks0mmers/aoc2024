use std::collections::HashSet;

use pathfinding::prelude::astar;
use utils::{AdventOfCode, Direction, Vec2};

struct MemorySpace {
    incoming: Vec<Vec2>,
    space: HashSet<Vec2>,
}

impl MemorySpace {
    const SIZE: Vec2 = if cfg!(test) {
        Vec2::new(6, 6)
    } else {
        Vec2::new(70, 70)
    };

    fn new(input: &str) -> Self {
        Self {
            incoming: input
                .lines()
                .filter_map(|line| {
                    line.split_once(",")
                        .and_then(|(x, y)| utils::parse_tuple::<usize>(x, y).ok())
                        .map(|pair| pair.into())
                })
                .collect(),
            space: HashSet::new(),
        }
    }

    fn simulate(&mut self, bytes: usize) {
        self.space.extend(&self.incoming[..bytes]);
        self.incoming = self.incoming[bytes..].to_vec();
    }

    fn drop_next(&mut self) -> Option<Vec2> {
        if let Some((left, rest)) = self.incoming.clone().split_first() {
            self.space.insert(left.clone());
            self.incoming = rest.to_vec();
            return Some(*left);
        }
        return None;
    }

    fn in_bounds(&self, pos: &Vec2) -> bool {
        return (0..=Self::SIZE.x).contains(&pos.x) && (0..=Self::SIZE.y).contains(&pos.y);
    }

    fn successors(&self, pos: Vec2) -> Vec<(Vec2, u32)> {
        Direction::all()
            .iter()
            .map(|dir| pos + dir.offset())
            .filter(|pos| self.in_bounds(&pos) && !self.space.contains(&pos))
            .map(|pos| (pos, 1))
            .collect()
    }

    fn distance(&self, pos: Vec2) -> u32 {
        let diff = Self::SIZE - pos;
        return (diff.x + diff.y) as u32;
    }

    fn shortest_path(&self) -> Option<usize> {
        return astar(
            &Vec2::ZERO,
            |p| self.successors(*p),
            |p| self.distance(*p),
            |p| *p == Self::SIZE,
        )
        .map(|(_, len)| len as usize);
    }
}

struct Day18;

impl AdventOfCode for Day18 {
    type Output = String;

    fn part1(input: &str) -> Self::Output {
        let mut mem = MemorySpace::new(input);
        mem.simulate(if cfg!(test) { 12 } else { 1024 });
        return mem.shortest_path().unwrap().to_string();
    }

    fn part2(input: &str) -> Self::Output {
        let mut mem = MemorySpace::new(input);
        mem.simulate(if cfg!(test) { 12 } else { 1024 });
        while let Some(curr) = mem.drop_next() {
            if mem.shortest_path().is_none() {
                return format!("{},{}", curr.x, curr.y);
            }
        }
        return "".to_string();
    }
}

fn main() {
    Day18::run(18);
}

#[cfg(test)]
mod tests {
    use crate::Day18;
    use utils::AdventOfCode;

    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn day18_part1() {
        let res = Day18::part1(INPUT);
        assert_eq!(res, "22");
    }

    #[test]
    fn day18_part2() {
        let res = Day18::part2(INPUT);
        assert_eq!(res, "6,1");
    }
}
