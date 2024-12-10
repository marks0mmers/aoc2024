use std::{env, fmt::Display, fs};

pub trait AdventOfCode {
    type Output;

    fn part1(input: &str) -> Self::Output;
    fn part2(input: &str) -> Self::Output;

    fn read_input_file(day: usize) -> std::io::Result<String> {
        let mut current = env::current_dir()?;
        current.push(format!("day{day}"));
        current.push("input.txt");
        fs::read_to_string(current.to_str().unwrap())
    }

    fn run(day: usize)
    where
        Self::Output: Display,
    {
        let input = Self::read_input_file(day).expect("failed to open input");
        println!("Part 1: {}", Self::part1(&input));
        println!("Part 2: {}", Self::part2(&input));
    }
}
