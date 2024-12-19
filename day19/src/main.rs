use std::collections::HashMap;

use utils::AdventOfCode;

struct Onsen<'a> {
    patterns: Vec<&'a str>,
    designs: Vec<&'a str>,
}

impl<'a> Onsen<'a> {
    fn new(input: &'a str) -> Self {
        let (patterns, designs) = input.split_once("\n\n").unwrap();
        Self {
            patterns: patterns.split(", ").collect(),
            designs: designs.lines().collect(),
        }
    }

    fn num_designs<'b>(&self, design: &'b str, cache: &mut HashMap<&'b str, usize>) -> usize {
        if design.is_empty() {
            return 1;
        }
        if let Some(&result) = cache.get(design) {
            return result;
        }
        let result = self
            .patterns
            .iter()
            .filter_map(|&pat| match design.starts_with(pat) {
                true => Some(self.num_designs(&design[pat.len()..], cache)),
                false => None,
            })
            .sum();

        cache.insert(design, result);
        return result;
    }
}

struct Day19;

impl AdventOfCode for Day19 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let onsen = Onsen::new(input);
        return onsen
            .designs
            .iter()
            .filter(|design| onsen.num_designs(design, &mut HashMap::new()) > 0)
            .count();
    }

    fn part2(input: &str) -> Self::Output {
        let onsen = Onsen::new(input);
        return onsen
            .designs
            .iter()
            .map(|design| onsen.num_designs(design, &mut HashMap::new()))
            .sum();
    }
}

fn main() {
    Day19::run(19);
}

#[cfg(test)]
mod tests {
    use crate::Day19;
    use utils::AdventOfCode;

    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn day19_part1() {
        let res = Day19::part1(INPUT);
        assert_eq!(res, 6);
    }

    #[test]
    fn day19_part2() {
        let res = Day19::part2(INPUT);
        assert_eq!(res, 16);
    }
}
