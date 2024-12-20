use std::{
    fmt::Display,
    isize,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    pub const ZERO: Point = Point::new(0, 0);

    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn from_pair(pair: (isize, isize)) -> Self {
        Self {
            x: pair.0,
            y: pair.1,
        }
    }

    pub fn xy(v: isize) -> Self {
        Self { x: v, y: v }
    }

    pub fn add_x(&self, x: isize) -> Point {
        Self {
            x: self.x + x,
            y: self.y,
        }
    }
    pub fn add_y(&self, y: isize) -> Point {
        Self {
            x: self.x,
            y: self.y + y,
        }
    }

    pub fn rect_dist(&self, other: Point) -> isize {
        let diff = *self - other;
        return diff.x.abs() + diff.y.abs();
    }
}

impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0 as isize, value.1 as isize)
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0 as isize, value.1 as isize)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<isize> for Point {
    type Output = Point;

    fn div(self, rhs: isize) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}
