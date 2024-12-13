use std::collections::{HashMap, HashSet};

use utils::{AdventOfCode, Direction, Vec2};

#[derive(Debug)]
struct Region {
    plots: HashSet<Vec2>,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        return self
            .plots
            .iter()
            .map(|plot| {
                Direction::all()
                    .iter()
                    .map(|dir| *plot + dir.get_offset())
                    .filter(|pos| !self.plots.contains(pos))
                    .count()
            })
            .sum();
    }

    fn sides(&self) -> usize {
        match self.plots.len() {
            1 => 4,
            _ => self
                .plots
                .iter()
                .map(|plot| {
                    let adj = Direction::all()
                        .iter()
                        .map(|dir| *plot + dir.get_offset())
                        .filter(|pos| self.plots.contains(pos))
                        .collect::<Vec<_>>();

                    return match adj.as_slice() {
                        [_] => 2,
                        [one, two] if one.x != two.x && one.y != two.y => {
                            let diag = *plot + *one - *plot + *two - *plot;
                            match !self.plots.contains(&diag) {
                                true => 2,
                                false => 1,
                            }
                        }
                        adj => {
                            let mut counts = HashMap::new();
                            for pos in adj.iter().flat_map(|pos| {
                                Direction::all()
                                    .iter()
                                    .map(|dir| *pos + dir.get_offset())
                                    .filter(|pos| !self.plots.contains(pos))
                                    .collect::<Vec<_>>()
                            }) {
                                counts
                                    .entry(pos)
                                    .and_modify(|count| *count += 1usize)
                                    .or_insert(1usize);
                            }

                            return counts.values().filter(|c| **c > 1).count();
                        }
                    };
                })
                .sum(),
        }
    }
}

fn find_adjacent<'a>(
    c: &'a char,
    plots: &'a HashMap<Vec2, char>,
    pos: &'a Vec2,
    existing: &'a mut HashSet<Vec2>,
) {
    existing.insert(*pos);

    for (pos, other_c) in Direction::all()
        .iter()
        .filter_map(|dir| plots.get_key_value(&(*pos + dir.get_offset())))
    {
        if other_c == c && !existing.contains(pos) {
            find_adjacent(other_c, plots, pos, existing)
        }
    }
}

fn parse_regions(input: &str) -> Vec<Region> {
    let plots: HashMap<Vec2, char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x, y).into(), c))
        })
        .collect();

    let mut points_to_skip: HashSet<Vec2> = HashSet::new();
    let mut regions = Vec::new();

    for (pos, c) in &plots {
        if points_to_skip.contains(&pos) {
            continue;
        }
        let mut region = Region {
            plots: HashSet::from([*pos]),
        };

        find_adjacent(c, &plots, pos, &mut region.plots);

        points_to_skip.extend(region.plots.iter());

        regions.push(region);
    }
    return regions;
}

struct Day12;

impl AdventOfCode for Day12 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        return parse_regions(input)
            .iter()
            .map(|region| region.area() * region.perimeter())
            .sum();
    }

    fn part2(input: &str) -> Self::Output {
        return parse_regions(input)
            .iter()
            .map(|region| region.area() * region.sides())
            .sum();
    }
}

fn main() {
    Day12::run(12);
}

#[cfg(test)]
mod tests {
    use crate::Day12;
    use utils::AdventOfCode;

    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn day12_part1() {
        let res = Day12::part1(INPUT);
        assert_eq!(res, 1930);
    }

    #[test]
    fn day12_part2() {
        let res = Day12::part2(INPUT);
        assert_eq!(res, 1206);
    }
}
