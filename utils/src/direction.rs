use crate::vec2::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
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

    pub fn get_offset(&self) -> Vec2 {
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
}
