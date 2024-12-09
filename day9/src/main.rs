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
    let mut result = blocks.clone();

    let mut blocks_rev = blocks
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, id)| id.is_some());

    for (i, c) in blocks.iter().enumerate() {
        if c.is_none() {
            if let Some((j, num)) = blocks_rev.next() {
                if i < j {
                    result[i] = *num;
                    result[j] = None;
                } else {
                    break;
                }
            }
        }
    }

    return result
        .iter()
        .enumerate()
        .filter_map(|(i, id_opt)| id_opt.map(|num| (i, num)))
        .map(|(i, id)| id * i)
        .sum();
}

#[derive(Clone, Copy, Debug)]
struct Block {
    id: usize,
    start: usize,
    end: usize,
}

impl Block {
    fn len(&self) -> usize {
        self.end - self.start
    }
}

fn part2(input: &str) -> usize {
    let mut blocks = Vec::new();

    let mut id = 0;
    let mut cursor = 0;
    for (i, c) in input.trim().chars().enumerate() {
        let num = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            let block = Block {
                id,
                start: cursor,
                end: cursor + num,
            };
            blocks.push(block);
            id += 1;
        }
        cursor += num;
    }

    for block in blocks.clone().iter().rev() {
        for j in 0..blocks.len() - 1 {
            let left = blocks[j];
            let right = blocks[j + 1];
            let len = block.len();
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
