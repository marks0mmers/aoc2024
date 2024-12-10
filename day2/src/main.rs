use utils::AdventOfCode;

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
    let mut diffs = report.windows(2).map(|pair| pair[0] - pair[1]);

    let diff_sign = (report[0] - report[1]).signum();

    return diffs.all(|diff| (1..4).contains(&diff.abs()) && diff.signum() == diff_sign);
}

struct Day2;

impl AdventOfCode for Day2 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let reports = parse_input(input);

        return reports.iter().filter(|report| is_safe(report)).count();
    }

    fn part2(input: &str) -> Self::Output {
        let reports = parse_input(input);

        return reports
            .iter()
            .filter(|report| {
                is_safe(report)
                    || (0..report.len())
                        .any(|i| is_safe(&[&report[0..i], &report[i + 1..]].concat()))
            })
            .count();
    }
}

fn main() {
    Day2::run(2);
}

#[cfg(test)]
mod tests {
    use crate::Day2;
    use utils::AdventOfCode;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn day2_part1() {
        let res = Day2::part1(INPUT);
        assert_eq!(res, 2);
    }

    #[test]
    fn day2_part2() {
        let res = Day2::part2(INPUT);
        assert_eq!(res, 4);
    }
}
