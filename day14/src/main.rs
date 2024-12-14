use std::collections::HashMap;

use image::{Rgb, RgbImage};
use utils::{AdventOfCode, Vec2};

struct State {
    robots: Vec<Robot>,
    size: Vec2,
}

impl State {
    fn new(input: &str) -> Self {
        Self {
            robots: input.lines().map(|line| Robot::new(line)).collect(),
            size: if cfg!(test) {
                Vec2::new(11, 7)
            } else {
                Vec2::new(101, 103)
            },
        }
    }

    fn r#move(&mut self) {
        for robot in self.robots.iter_mut() {
            robot.r#move(self.size);
        }
    }

    fn pos_map(&self) -> HashMap<Vec2, usize> {
        let mut map = HashMap::new();

        for robot in &self.robots {
            map.entry(robot.pos)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        return map;
    }

    fn output_image(&self, second: usize) {
        let mut image = RgbImage::new(self.size.x as u32, self.size.y as u32);
        let pos_map = self.pos_map();

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                if let Some(_) = pos_map.get(&(x, y).into()) {
                    image.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]));
                }
            }
        }

        image
            .save(format!("./day14/output/{second}.png"))
            .expect("failed to save");
    }
}

struct Robot {
    pos: Vec2,
    vel: Vec2,
}

impl Robot {
    fn new(input: &str) -> Self {
        let (pos, vel) = input.split_once(" ").unwrap();
        let pos = &pos[2..];
        let vel = &vel[2..];
        let (px, py) = pos.split_once(",").unwrap();
        let (vx, vy) = vel.split_once(",").unwrap();
        let (px, py) = utils::parse_tuple(px, py).unwrap();
        let (vx, vy) = utils::parse_tuple(vx, vy).unwrap();
        Self {
            pos: Vec2::new(px, py),
            vel: Vec2::new(vx, vy),
        }
    }

    fn r#move(&mut self, bounds: Vec2) {
        self.pos = self.pos + self.vel;
        if self.pos.x < 0 {
            self.pos.x += bounds.x;
        }
        if self.pos.x >= bounds.x {
            self.pos.x -= bounds.x;
        }
        if self.pos.y < 0 {
            self.pos.y += bounds.y;
        }
        if self.pos.y >= bounds.y {
            self.pos.y -= bounds.y;
        }
    }
}

struct Day14;

impl AdventOfCode for Day14 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let mut state = State::new(input);
        for _ in 0..100 {
            state.r#move();
        }
        let pos_map = state.pos_map();

        let width = state.size / 2;
        let quadrants = (0..2)
            .flat_map(|x| {
                (0..2).map(move |y| {
                    let dx = match x {
                        0 => 0,
                        x => width.x * x + 1,
                    };
                    let dy = match y {
                        0 => 0,
                        y => width.y * y + 1,
                    };
                    (Vec2::ZERO.add_x(dx).add_y(dy), width.add_x(dx).add_y(dy))
                })
            })
            .collect::<Vec<_>>();

        return quadrants
            .iter()
            .map(|(start, end)| {
                pos_map
                    .iter()
                    .filter(|(pos, _)| {
                        (start.x..end.x).contains(&pos.x) && (start.y..end.y).contains(&pos.y)
                    })
                    .map(|(_, count)| count)
                    .sum::<usize>()
            })
            .fold(1, |acc, val| acc * val);
    }

    fn part2(input: &str) -> Self::Output {
        let mut state = State::new(input);
        for i in 0..10000 {
            state.r#move();
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
