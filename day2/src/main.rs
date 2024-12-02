use std::fs;

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    return input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|chars| chars.parse::<isize>().expect("invalid number"))
                .collect()
        })
        .collect();
}

fn is_safe(report: &[isize]) -> bool {
    let diffs = report.windows(2).map(|pair| pair[0] - pair[1]);

    let is_negative = (report[0] - report[1]).is_negative();

    for diff in diffs {
        if !(1..=3).contains(&diff.abs())
            || is_negative && diff.is_positive()
            || !is_negative && diff.is_negative()
        {
            return false;
        }
    }

    return true;
}

fn part1(input: &str) -> usize {
    let reports = parse_input(input);

    return reports.iter().filter(|report| is_safe(report)).count();
}

fn part2(input: &str) -> usize {
    let reports = parse_input(input);

    return reports
        .iter()
        .filter(|report| {
            is_safe(report)
                || (0..report.len()).any(|i| is_safe(&[&report[0..i], &report[i + 1..]].concat()))
        })
        .count();
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to open input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        let res = part1(INPUT);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_part2() {
        let res = part2(INPUT);
        assert_eq!(res, 4);
    }
}
