use pathfinding::prelude::astar;
use utils::{AdventOfCode, Direction, Point};

struct MemorySpace {
    incoming: Vec<Point>,
}

impl MemorySpace {
    const SIZE: Point = if cfg!(test) {
        Point::new(6, 6)
    } else {
        Point::new(70, 70)
    };

    fn new(input: &str) -> Self {
        Self {
            incoming: input
                .lines()
                .filter_map(|line| {
                    line.split_once(",")
                        .and_then(|(x, y)| utils::parse_tuple(x, y).ok())
                        .map(|pair| Point::from_pair(pair))
                })
                .collect(),
        }
    }

    fn in_bounds(&self, pos: &Point) -> bool {
        return (0..=Self::SIZE.x).contains(&pos.x) && (0..=Self::SIZE.y).contains(&pos.y);
    }

    fn shortest_path(&self, bytes: usize) -> Option<usize> {
        return astar(
            &Point::ZERO,
            |&pos| {
                Direction::all()
                    .iter()
                    .map(|dir| pos + dir.offset())
                    .filter(|pos| self.in_bounds(&pos) && !self.incoming[..bytes].contains(&pos))
                    .map(|pos| (pos, 1))
                    .collect::<Vec<_>>()
            },
            |&pos| {
                let diff = Self::SIZE - pos;
                return (diff.x + diff.y) as usize;
            },
            |&pos| pos == Self::SIZE,
        )
        .map(|(_, len)| len);
    }
}

struct Day18;

impl AdventOfCode for Day18 {
    type Output = String;

    fn part1(input: &str) -> Self::Output {
        let mem = MemorySpace::new(input);
        return mem
            .shortest_path(if cfg!(test) { 12 } else { 1024 })
            .unwrap()
            .to_string();
    }

    fn part2(input: &str) -> Self::Output {
        let mem = MemorySpace::new(input);
        for i in if cfg!(test) { 12 } else { 1024 }..mem.incoming.len() {
            if mem.shortest_path(i).is_none() {
                let curr = mem.incoming[i - 1];
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
