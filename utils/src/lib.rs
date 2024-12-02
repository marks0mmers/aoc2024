use std::{env, fs};

pub fn read_input_file(day: usize) -> std::io::Result<String> {
    let mut current = env::current_dir()?;
    current.push(format!("day{day}"));
    current.push("input.txt");
    fs::read_to_string(current.to_str().unwrap())
}
