use std::collections::HashMap;

use utils::{AdventOfCode, Direction, Point};

#[derive(Debug, Clone, Copy)]
enum BoxSide {
    Left,
    Right,
}

impl BoxSide {
    fn offset(&self) -> isize {
        match self {
            BoxSide::Left => 1,
            BoxSide::Right => -1,
        }
    }

    fn opposite(&self) -> Self {
        match self {
            BoxSide::Left => BoxSide::Right,
            BoxSide::Right => BoxSide::Left,
        }
    }
}

#[derive(Debug)]
enum Tile {
    Wall,
    Box(BoxSide),
}

impl Tile {
    fn new(c: char) -> Option<Self> {
        match c {
            '#' => Some(Self::Wall),
            'O' | '[' => Some(Self::Box(BoxSide::Left)),
            ']' => Some(Self::Box(BoxSide::Right)),
            _ => None,
        }
    }
}

struct Warehouse {
    tiles: HashMap<Point, Tile>,
    moves: Vec<Direction>,
    robot: Point,
}

impl Warehouse {
    fn part1(input: &str) -> Self {
        let (tiles, moves) = input.split_once("\n\n").unwrap();

        let (robot, _) = tiles
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x, y).into(), c))
            })
            .find(|(_, c)| *c == '@')
            .unwrap();

        let tiles = tiles
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, c)| Tile::new(c).map(|tile| ((x, y).into(), tile)))
            })
            .collect();

        let moves = moves
            .chars()
            .filter_map(|c| match c {
                '^' => Some(Direction::North),
                '>' => Some(Direction::East),
                'v' => Some(Direction::South),
                '<' => Some(Direction::West),
                _ => None,
            })
            .collect();

        Self {
            tiles,
            moves,
            robot,
        }
    }

    fn part2(input: &str) -> Self {
        Self::part1(
            &input
                .replace("#", "##")
                .replace("O", "[]")
                .replace(".", "..")
                .replace("@", "@."),
        )
    }

    fn can_move(&self, pos: Point, dir: &Direction, expanded: bool) -> bool {
        let next_pos = pos + dir.offset();
        match self.tiles.get(&next_pos) {
            Some(Tile::Box(_)) if !expanded || dir.is_horizontal() => {
                self.can_move(next_pos, dir, expanded)
            }
            Some(Tile::Box(side)) => {
                self.can_move(next_pos, dir, expanded)
                    && self.can_move(next_pos.add_x(side.offset()), dir, expanded)
            }
            Some(Tile::Wall) => false,
            _ => true,
        }
    }

    fn do_move(&mut self, pos: Point, dir: &Direction, expanded: bool) {
        let next_pos = pos + dir.offset();
        match self.tiles.remove(&pos) {
            Some(Tile::Box(side)) if !expanded || dir.is_horizontal() => {
                self.do_move(next_pos, dir, expanded);
                self.tiles.insert(next_pos, Tile::Box(side));
            }
            Some(Tile::Box(side)) => {
                self.tiles.remove(&pos.add_x(side.offset()));
                self.do_move(next_pos, dir, expanded);
                self.do_move(next_pos.add_x(side.offset()), dir, expanded);
                self.tiles.insert(next_pos, Tile::Box(side));
                self.tiles
                    .insert(next_pos.add_x(side.offset()), Tile::Box(side.opposite()));
            }
            _ => {}
        }
    }

    fn execute(&mut self, expanded: bool) {
        let moves = self.moves.clone();
        for dir in moves {
            if self.can_move(self.robot, &dir, expanded) {
                self.do_move(self.robot + dir.offset(), &dir, expanded);
                self.robot = self.robot + dir.offset();
            }
        }
    }

    fn gps_coords(&self) -> usize {
        self.tiles
            .iter()
            .filter_map(|(pos, tile)| match tile {
                Tile::Box(BoxSide::Left) => Some((pos.x + pos.y * 100) as usize),
                _ => None,
            })
            .sum()
    }
}

struct Day15;

impl AdventOfCode for Day15 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let mut warehouse = Warehouse::part1(input);
        warehouse.execute(false);
        return warehouse.gps_coords();
    }

    fn part2(input: &str) -> Self::Output {
        let mut warehouse = Warehouse::part2(input);
        warehouse.execute(true);
        return warehouse.gps_coords();
    }
}

fn main() {
    Day15::run(15);
}

#[cfg(test)]
mod tests {
    use crate::Day15;
    use utils::AdventOfCode;

    const INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn day15_part1() {
        let res = Day15::part1(INPUT);
        assert_eq!(res, 10092);
    }

    #[test]
    fn day15_part2() {
        let res = Day15::part2(INPUT);
        assert_eq!(res, 9021);
    }
}
