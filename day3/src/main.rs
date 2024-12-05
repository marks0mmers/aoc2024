use regex::Regex;

fn part1(input: &str) -> usize {
    let mut total = 0;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for (_, [left, right]) in re.captures_iter(input).map(|c| c.extract()) {
        if let (Ok(left), Ok(right)) = (left.parse::<usize>(), right.parse::<usize>()) {
            total += left * right;
        }
    }

    return total;
}

fn part2(input: &str) -> usize {
    let mut total = 0;
    let mut should_mul = true;

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    for c in re.captures_iter(input) {
        match c.get(0) {
            Some(m) if m.as_str() == "do()" => should_mul = true,
            Some(m) if m.as_str() == "don't()" => should_mul = false,
            Some(_) if should_mul => {
                let left = c.get(1).and_then(|m| m.as_str().parse::<usize>().ok());
                let right = c.get(2).and_then(|m| m.as_str().parse::<usize>().ok());

                if let (Some(left), Some(right)) = (left, right) {
                    total += left * right;
                }
            }
            _ => {}
        }
    }

    return total;
}

fn main() {
    let input = utils::read_input_file(3).expect("failed to open input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn day3_part1() {
        let res = part1(INPUT);
        assert_eq!(res, 161);
    }

    #[test]
    fn day3_part2() {
        let res = part2(INPUT);
        assert_eq!(res, 48);
    }
}
