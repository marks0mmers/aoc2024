use std::{cmp::Ordering, collections::HashMap};

struct Update {
    pages: Vec<usize>,
}

impl Update {
    fn new(pages: Vec<usize>) -> Self {
        Self { pages }
    }

    fn is_in_order(&self, ordering: &HashMap<usize, Vec<usize>>) -> bool {
        for (i, page) in self.pages.iter().enumerate() {
            if let Some(ordering_for_page) = ordering.get(page) {
                let remaining_pages = &self.pages[i + 1..];
                if ordering_for_page
                    .iter()
                    .any(|order| self.pages.contains(order) && !remaining_pages.contains(order))
                {
                    return false;
                }
            }
        }

        return true;
    }

    fn fix_order(&self, ordering: &HashMap<usize, Vec<usize>>) -> Update {
        let mut new_pages = self.pages.clone();
        new_pages.sort_by(|a, b| {
            if let Some(ordering_for_page_a) = ordering.get(a) {
                if ordering_for_page_a.contains(b) {
                    return Ordering::Less;
                }
            }
            if let Some(ordering_for_page_b) = ordering.get(b) {
                if ordering_for_page_b.contains(a) {
                    return Ordering::Greater;
                }
            }

            return Ordering::Equal;
        });
        return Update { pages: new_pages };
    }

    fn middle_page(&self) -> usize {
        return self.pages[self.pages.len() / 2];
    }
}

struct SafetyManual {
    ordering: HashMap<usize, Vec<usize>>,
    updates: Vec<Update>,
}

impl SafetyManual {
    fn new(input: &str) -> Self {
        let (ordering_string, update_string) = input.split_once("\n\n").expect("invalid input");

        let mut ordering = HashMap::new();

        for (key, value) in ordering_string
            .lines()
            .filter_map(|line| line.split_once("|"))
            .map(|(left, right)| {
                (
                    left.parse::<usize>().expect("invalid number"),
                    right.parse::<usize>().expect("invalid number"),
                )
            })
        {
            ordering
                .entry(key)
                .and_modify(|values: &mut Vec<usize>| values.extend([value]))
                .or_insert(vec![value]);
        }

        let updates = update_string
            .lines()
            .map(|line| {
                Update::new(
                    line.split(",")
                        .filter_map(|num_str| num_str.parse::<usize>().ok())
                        .collect(),
                )
            })
            .collect::<Vec<_>>();

        Self { ordering, updates }
    }

    fn get_updates_in_order(&self) -> Vec<&Update> {
        self.updates
            .iter()
            .filter(|update| update.is_in_order(&self.ordering))
            .collect()
    }

    fn get_updates_out_of_order(&self) -> Vec<&Update> {
        self.updates
            .iter()
            .filter(|update| !update.is_in_order(&self.ordering))
            .collect()
    }
}

fn part1(input: &str) -> usize {
    let manual = SafetyManual::new(input);

    return manual
        .get_updates_in_order()
        .iter()
        .map(|update| update.middle_page())
        .sum();
}

fn part2(input: &str) -> usize {
    let manual = SafetyManual::new(input);

    return manual
        .get_updates_out_of_order()
        .iter()
        .map(|update| update.fix_order(&manual.ordering).middle_page())
        .sum();
}

fn main() {
    let input = utils::read_input_file(5).expect("failed to open input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn day5_part1() {
        let res = part1(INPUT);
        assert_eq!(res, 143);
    }

    #[test]
    fn day5_part2() {
        let res = part2(INPUT);
        assert_eq!(res, 123);
    }
}
