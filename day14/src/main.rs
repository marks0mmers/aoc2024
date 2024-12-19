use std::collections::HashMap;

use image::{Rgb, RgbImage};
use utils::{AdventOfCode, Point};

struct State {
    robots: Vec<Robot>,
    size: Point,
}

impl State {
    fn new(input: &str) -> Self {
        Self {
            robots: input.lines().filter_map(|line| Robot::new(line)).collect(),
            size: if cfg!(test) {
                Point::new(11, 7)
            } else {
                Point::new(101, 103)
            },
        }
    }

    fn tick(&mut self) {
        self.robots.iter_mut().for_each(|r| r.tick(self.size));
    }

    fn pos_map(&self) -> HashMap<Point, usize> {
        return self.robots.iter().fold(HashMap::new(), |mut map, r| {
            map.entry(r.pos)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            map
        });
    }

    fn output_image(&self, second: usize) {
        let mut image = RgbImage::new(self.size.x as u32, self.size.y as u32);
        let pos_map = self.pos_map();

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                pos_map
                    .get(&(x, y).into())
                    .inspect(|_| image.put_pixel(x as u32, y as u32, Rgb([255, 255, 255])));
            }
        }

        image
            .save(format!("./day14/output/{second}.png"))
            .expect("failed to save");
    }
}

struct Robot {
    pos: Point,
    vel: Point,
}

impl Robot {
    fn new(input: &str) -> Option<Self> {
        let (pos, vel) = input.split_once(" ")?;
        let pos = &pos[2..];
        let vel = &vel[2..];
        let (px, py) = pos.split_once(",")?;
        let (vx, vy) = vel.split_once(",")?;
        let (px, py) = utils::parse_tuple(px, py).ok()?;
        let (vx, vy) = utils::parse_tuple(vx, vy).ok()?;
        Some(Self {
            pos: Point::new(px, py),
            vel: Point::new(vx, vy),
        })
    }

    fn tick(&mut self, bounds: Point) {
        self.pos = self.pos + self.vel;
        if !(0..bounds.x).contains(&self.pos.x) {
            self.pos.x -= self.pos.x.signum() * bounds.x;
        }
        if !(0..bounds.y).contains(&self.pos.y) {
            self.pos.y -= self.pos.y.signum() * bounds.y;
        }
    }
}

struct Day14;

impl AdventOfCode for Day14 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let mut state = State::new(input);
        (0..100).for_each(|_| state.tick());

        let pos_map = state.pos_map();
        let width = state.size / 2;

        return (0..2)
            .flat_map(|x| {
                (0..2).map(move |y| {
                    let dx = if x != 0 { width.x * x + 1 } else { 0 };
                    let dy = if y != 0 { width.y * y + 1 } else { 0 };
                    (Point::ZERO.add_x(dx).add_y(dy), width.add_x(dx).add_y(dy))
                })
            })
            .map(|(start, end)| {
                pos_map
                    .iter()
                    .filter(|(pos, _)| {
                        (start.x..end.x).contains(&pos.x) && (start.y..end.y).contains(&pos.y)
                    })
                    .map(|(_, count)| count)
                    .sum::<usize>()
            })
            .product();
    }

    fn part2(input: &str) -> Self::Output {
        let mut state = State::new(input);
        for i in 0..10000 {
            state.tick();
            if !cfg!(test) {
                state.output_image(i + 1);
            }
        }
        return 0;
    }
}

fn main() {
    Day14::run(14);
}

#[cfg(test)]
mod tests {
    use crate::Day14;
    use utils::AdventOfCode;

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn day14_part1() {
        let res = Day14::part1(INPUT);
        assert_eq!(res, 12);
    }

    #[test]
    fn day14_part2() {
        let res = Day14::part2(INPUT);
        assert_eq!(res, 0);
    }
}
