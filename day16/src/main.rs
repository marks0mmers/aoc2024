use std::collections::{HashMap, HashSet, VecDeque};

use utils::{AdventOfCode, Direction, Vec2};

struct Maze {
    tiles: HashSet<Vec2>,
    reindeer: (Vec2, Direction),
    end: Vec2,
}

#[derive(Hash, PartialEq, Eq)]
struct Key {
    pos: Vec2,
    dir: Direction,
}

impl Maze {
    fn new(input: &str) -> Self {
        let chars_iter = input.lines().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x, y).into(), c))
        });

        let (start, _) = chars_iter.clone().find(|(_, c)| *c == 'S').unwrap();

        let (end, _) = chars_iter.clone().find(|(_, c)| *c == 'E').unwrap();

        let tiles = chars_iter
            .clone()
            .filter_map(|(pos, c)| match c {
                '#' => Some(pos),
                _ => None,
            })
            .collect::<HashSet<_>>();

        Self {
            tiles,
            reindeer: (start, Direction::East),
            end,
        }
    }

    fn best_solution(&self) -> usize {
        let mut visited = HashMap::new();
        let mut min_score = usize::MAX;

        let mut queue = VecDeque::from([(self.reindeer.0, self.reindeer.1, 0usize)]);
        while let Some((pos, dir, score)) = queue.pop_front() {
            if visited
                .get(&Key { pos, dir })
                .is_some_and(|cost| *cost < score)
            {
                continue;
            }
            visited.insert(Key { pos, dir }, score);
            let front = pos + dir.offset();
            let left = dir.turn_left();
            let right = dir.turn_right();

            if front == self.end {
                min_score = min_score.min(score + 1);
            }
            if !self.tiles.contains(&front) {
                queue.push_back((front, dir, score + 1));
            }
            if !self.tiles.contains(&(pos + left.offset())) {
                queue.push_back((pos, left, score + 1000));
            }
            if !self.tiles.contains(&(pos + right.offset())) {
                queue.push_back((pos, right, score + 1000));
            }
        }

        return min_score;
    }

    fn all_paths(&self) -> usize {
        let pos = self.reindeer.0;
        let mut visited = HashMap::new();
        let mut min_score = usize::MAX;
        let mut score_map = HashMap::new();

        let mut queue = VecDeque::from([(pos, self.reindeer.1, 0usize, HashSet::from([pos]))]);
        while let Some((pos, dir, score, path)) = queue.pop_front() {
            if visited
                .get(&Key { pos, dir })
                .is_some_and(|cost| *cost < score)
            {
                continue;
            }

            visited.insert(Key { pos, dir }, score);

            let front = pos + dir.offset();
            let left = dir.turn_left();
            let right = dir.turn_right();
            let mut new_path = path.clone();
            new_path.insert(front);

            if front == self.end {
                let score = score + 1;
                if score <= min_score {
                    min_score = min_score.min(score);
                    score_map
                        .entry(score)
                        .and_modify(|p: &mut HashSet<Vec2>| p.extend(&new_path))
                        .or_insert(new_path);
                }
                continue;
            }

            if !self.tiles.contains(&front) {
                queue.push_back((front, dir, score + 1, new_path));
            }
            if !self.tiles.contains(&(pos + left.offset())) {
                queue.push_back((pos, left, score + 1000, path.clone()));
            }
            if !self.tiles.contains(&(pos + right.offset())) {
                queue.push_back((pos, right, score + 1000, path.clone()));
            }
        }

        return score_map.get(&min_score).unwrap().len();
    }
}

struct Day16;

impl AdventOfCode for Day16 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        return Maze::new(input).best_solution();
    }

    fn part2(input: &str) -> Self::Output {
        return Maze::new(input).all_paths();
    }
}

fn main() {
    Day16::run(16);
}

#[cfg(test)]
mod tests {
    use crate::Day16;
    use utils::AdventOfCode;

    const INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn day16_part1() {
        let res = Day16::part1(INPUT);
        assert_eq!(res, 7036);
    }

    #[test]
    fn day16_part2() {
        let res = Day16::part2(INPUT);
        assert_eq!(res, 45);
    }
}
