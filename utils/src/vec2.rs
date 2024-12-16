use std::{
    fmt::Display,
    isize,
    ops::{Add, Div, Mul, Sub},
};

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
    pub const ZERO: Vec2 = Vec2::new(0, 0);

    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn xy(v: isize) -> Self {
        Self { x: v, y: v }
    }

    pub fn add_x(&self, x: isize) -> Vec2 {
        Self {
            x: self.x + x,
            y: self.y,
        }
    }
    pub fn add_y(&self, y: isize) -> Vec2 {
        Self {
            x: self.x,
            y: self.y + y,
        }
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

impl Mul<isize> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: isize) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<isize> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: isize) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}
