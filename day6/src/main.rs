use std::collections::HashMap;

use utils::{AdventOfCode, Direction, Vec2};

#[derive(Debug, Clone, Copy)]
enum Tile {
    Blocked,
    Empty,
    Visited(Direction),
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '#' => Self::Blocked,
            '.' => Self::Empty,
            '^' => Self::Visited(Direction::North),
            _ => panic!("invalid tile: {c}"),
        }
    }
}

#[derive(Clone, Copy)]
struct Guard {
    pos: Vec2,
    dir: Direction,
}

#[derive(Clone)]
struct State {
    guard: Guard,
    tiles: HashMap<Vec2, Tile>,
    size: Vec2,
}

impl State {
    fn new(input: &str) -> Self {
        let mut tiles = HashMap::new();
        let mut guard: Option<Guard> = None;

        let size = (input.lines().next().unwrap().len(), input.lines().count()).into();

        for (y, row) in input.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let tile = Tile::new(c);
                tiles.insert((x, y).into(), tile);

                if c == '^' {
                    guard = Some(Guard {
                        pos: (x, y).into(),
                        dir: Direction::North,
                    })
                }
            }
        }

        Self {
            tiles,
            size,
            guard: guard.expect("failed to find guard"),
        }
    }

    fn take_turn(&mut self) -> bool {
        if self.guard.pos.x < 0
            || self.guard.pos.x >= self.size.x
            || self.guard.pos.y < 0
            || self.guard.pos.y >= self.size.y
        {
            return false;
        }
        let next_pos = self.guard.pos + self.guard.dir.get_offset();

        match self.tiles.get(&next_pos) {
            Some(next) if matches!(next, Tile::Blocked) => {
                self.guard.dir = self.guard.dir.turn_right()
            }
            Some(next) if matches!(next, Tile::Empty) => {
                self.tiles.insert(next_pos, Tile::Visited(self.guard.dir));
                self.guard.pos = next_pos;
            }
            _ => self.guard.pos = next_pos,
        }

        return true;
    }

    fn is_blocked(&self, pos: Vec2) -> bool {
        self.tiles
            .get(&pos)
            .is_some_and(|tile| matches!(tile, Tile::Blocked))
    }

    fn is_loop(&self, pos: Vec2) -> bool {
        let mut new_state = self.clone();
        new_state.tiles.insert(pos, Tile::Blocked);

        loop {
            if new_state.guard.pos.x < 0
                || new_state.guard.pos.x >= new_state.size.x
                || new_state.guard.pos.y < 0
                || new_state.guard.pos.y >= new_state.size.y
            {
                return false;
            }
            let next_pos = new_state.guard.pos + new_state.guard.dir.get_offset();

            match new_state.tiles.get(&next_pos) {
                Some(next) => match next {
                    Tile::Blocked => new_state.guard.dir = new_state.guard.dir.turn_right(),
                    Tile::Empty => {
                        new_state
                            .tiles
                            .insert(next_pos, Tile::Visited(new_state.guard.dir));
                        new_state.guard.pos = next_pos;
                    }
                    Tile::Visited(dir) => {
                        if new_state.guard.dir == *dir {
                            return true;
                        } else {
                            new_state.guard.pos = next_pos;
                        }
                    }
                },
                _ => new_state.guard.pos = next_pos,
            }
        }
    }

    fn num_visited(&self) -> usize {
        self.tiles
            .values()
            .filter(|tile| matches!(tile, Tile::Visited(_)))
            .count()
    }
}

struct Day6;

impl AdventOfCode for Day6 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let mut state = State::new(input);
        loop {
            if !state.take_turn() {
                return state.num_visited();
            }
        }
    }

    fn part2(input: &str) -> Self::Output {
        let state = State::new(input);
        let mut total = 0;

        for x in 0..state.size.x {
            for y in 0..state.size.y {
                let pos = Vec2::new(x, y);
                if pos != state.guard.pos && !state.is_blocked(pos) && state.is_loop(pos) {
                    total += 1;
                }
            }
        }

        return total;
    }
}

fn main() {
    Day6::run(6);
}

#[cfg(test)]
mod tests {
    use crate::Day6;
    use utils::AdventOfCode;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn day6_part1() {
        let res = Day6::part1(INPUT);
        assert_eq!(res, 41);
    }

    #[test]
    fn day6_part2() {
        let res = Day6::part2(INPUT);
        assert_eq!(res, 6);
    }
}
