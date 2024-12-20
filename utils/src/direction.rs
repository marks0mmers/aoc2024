use std::fmt::Display;

use crate::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::East => write!(f, "East"),
            Direction::South => write!(f, "South"),
            Direction::West => write!(f, "West"),
        }
    }
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    pub fn offset(&self) -> Point {
        match self {
            Direction::North => (0, -1).into(),
            Direction::East => (1, 0).into(),
            Direction::South => (0, 1).into(),
            Direction::West => (-1, 0).into(),
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::North => Self::East,
            Direction::East => Self::South,
            Direction::South => Self::West,
            Direction::West => Self::North,
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Direction::North => Self::West,
            Direction::East => Self::North,
            Direction::South => Self::East,
            Direction::West => Self::South,
        }
    }

    pub fn is_north(&self) -> bool {
        matches!(self, Direction::North)
    }

    pub fn is_east(&self) -> bool {
        matches!(self, Direction::East)
    }

    pub fn is_south(&self) -> bool {
        matches!(self, Direction::South)
    }

    pub fn is_west(&self) -> bool {
        matches!(self, Direction::West)
    }

    pub fn is_horizontal(&self) -> bool {
        self.is_east() || self.is_west()
    }

    pub fn is_vertical(&self) -> bool {
        self.is_north() || self.is_south()
    }

    pub fn opposite(&self) -> Self {
        match self {
            Direction::North => Self::South,
            Direction::East => Self::West,
            Direction::South => Self::North,
            Direction::West => Self::East,
        }
    }
}
