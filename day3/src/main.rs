fn scan_mul(scanner: &str) -> Option<(&str, usize)> {
    let mut scanner = &scanner[4..];
    let left: String = scanner.chars().take_while(|c| c.is_numeric()).collect();
    scanner = &scanner[left.len()..];
    if scanner.starts_with(",") {
        scanner = &scanner[1..];
        let right: String = scanner.chars().take_while(|c| c.is_numeric()).collect();
        scanner = &scanner[right.len()..];
        if scanner.starts_with(")") {
            scanner = &scanner[1..];
            if let (Ok(left), Ok(right)) = (left.parse::<usize>(), right.parse::<usize>()) {
                return Some((scanner, left * right));
            }
        }
    }

    return None;
}

fn part1(input: &str) -> usize {
    let mut total = 0;
    let mut scanner = input;
    while scanner.len() > 0 {
        if scanner.starts_with("mul(") {
            if let Some((scanned, res)) = scan_mul(scanner) {
                scanner = scanned;
                total += res;
                continue;
            }
        }

        scanner = &scanner[1..];
    }
    return total;
}

fn part2(input: &str) -> usize {
    let mut total = 0;
    let mut should_mul = true;
    let mut scanner = input;
    while scanner.len() > 0 {
        if scanner.starts_with("don't()") {
            scanner = &scanner[7..];
            should_mul = false;
        }

        if scanner.starts_with("do()") {
            scanner = &scanner[4..];
            should_mul = true;
        }

        if should_mul && scanner.starts_with("mul(") {
            if let Some((scanned, res)) = scan_mul(scanner) {
                scanner = scanned;
                total += res;
                continue;
            }
        }

        scanner = &scanner[1..];
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
