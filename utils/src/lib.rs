use std::{
    env,
    fmt::Display,
    fs,
    ops::{Add, Sub},
};

pub fn read_input_file(day: usize) -> std::io::Result<String> {
    let mut current = env::current_dir()?;
    current.push(format!("day{day}"));
    current.push("input.txt");
    fs::read_to_string(current.to_str().unwrap())
}

pub trait AdventOfCode {
    type Output;

    fn part1(input: &str) -> Self::Output;
    fn part2(input: &str) -> Self::Output;

    fn run(day: usize)
    where
        Self::Output: Display,
    {
        let input = read_input_file(day).expect("failed to open input");
        println!("Part 1: {}", Self::part1(&input));
        println!("Part 2: {}", Self::part2(&input));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Vec2 {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl From<(isize, isize)> for Vec2 {
    fn from(value: (isize, isize)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0 as isize, value.1 as isize)
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0 as isize, value.1 as isize)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
