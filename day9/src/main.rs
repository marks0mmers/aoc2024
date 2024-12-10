use std::iter::repeat;

fn part1(input: &str) -> usize {
    let mut blocks = Vec::new();
    let mut id = 0;

    for (i, c) in input.trim().chars().enumerate() {
        let num = c.to_digit(10).unwrap() as usize;
        match i % 2 == 0 {
            true => {
                blocks.extend(repeat(Some(id)).take(num));
                id += 1;
            }
            false => blocks.extend(repeat(None).take(num)),
        }
    }

    let mut blocks_rev = blocks
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(i, id)| id.map(|id| (i, id)));

    let mut last_rev_read = blocks.len();

    return blocks
        .iter()
        .enumerate()
        .map_while(|(i, c)| {
            if i >= last_rev_read {
                return None;
            }
            match c {
                None => match blocks_rev.next() {
                    Some((j, id)) => {
                        last_rev_read = j;
                        Some(id * i)
                    }
                    None => None,
                },
                Some(id) => Some(id * i),
            }
        })
        .sum();
}

#[derive(Clone, Copy, Debug)]
struct Block {
    id: usize,
    start: usize,
    end: usize,
}

fn part2(input: &str) -> usize {
    let mut blocks = Vec::new();

    let mut id = 0;
    let mut cursor = 0;

    for (i, c) in input.trim().chars().enumerate() {
        let num = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            blocks.push(Block {
                id,
                start: cursor,
                end: cursor + num,
            });
            id += 1;
        }
        cursor += num;
    }

    for block in blocks.clone().iter().rev() {
        for j in 0..blocks.len() - 1 {
            let left = blocks[j];
            let right = blocks[j + 1];
            let len = block.end - block.start;
            if len <= right.start - left.end && block.start > left.start {
                let block = blocks.iter_mut().find(|b| b.id == block.id).unwrap();
                block.start = left.end;
                block.end = left.end + len;
                break;
            }
        }
        blocks.sort_by(|a, b| a.start.cmp(&b.start));
    }

    return blocks
        .iter()
        .map(|block| {
            (block.start..block.end)
                .map(|pos| pos * block.id)
                .sum::<usize>()
        })
        .sum();
}

fn main() {
    let input = utils::read_input_file(9).expect("failed to open input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn day9_part1() {
        let res = part1(INPUT);
        assert_eq!(res, 1928);
    }

    #[test]
    fn day9_part2() {
        let res = part2(INPUT);
        assert_eq!(res, 2858);
    }
}
