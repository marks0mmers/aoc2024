struct WordSearch {
    grid: Vec<Vec<char>>,
}

impl WordSearch {
    fn new(input: &str) -> Self {
        Self {
            grid: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn row_count(&self) -> usize {
        self.grid.len()
    }

    fn col_count(&self) -> usize {
        self.grid.first().unwrap().len()
    }

    fn num_of_xmas(&self, row: usize, col: usize) -> usize {
        let mut total = 0;
        // check left
        if col + 3 < self.col_count() {
            // check l-r
            if &self.grid[row][col..col + 4] == ['X', 'M', 'A', 'S'] {
                total += 1;
            }
            // check tl-br
            if row + 3 < self.row_count()
                && self.grid[row..row + 4]
                    .iter()
                    .enumerate()
                    .map(|(i, row)| row[col + i])
                    .collect::<String>()
                    == "XMAS"
            {
                total += 1;
            }

            // check bl-tr
            if row >= 3
                && self.grid[row - 3..row + 1]
                    .iter()
                    .enumerate()
                    .map(|(i, row)| row[col + 3 - i])
                    .collect::<String>()
                    == "SAMX"
            {
                total += 1;
            }
        }
        // check right
        if col >= 3 {
            // check r-l
            if &self.grid[row][col - 3..col + 1] == ['S', 'A', 'M', 'X'] {
                total += 1;
            }

            // check tr-bl
            if row + 3 < self.row_count()
                && self.grid[row..row + 4]
                    .iter()
                    .enumerate()
                    .map(|(i, row)| row[col - i])
                    .collect::<String>()
                    == "XMAS"
            {
                total += 1;
            }

            // check br-tl
            if row >= 3
                && self.grid[row - 3..row + 1]
                    .iter()
                    .enumerate()
                    .map(|(i, row)| row[col - 3 + i])
                    .collect::<String>()
                    == "SAMX"
            {
                total += 1;
            }
        }
        // check t-b
        if row + 3 < self.row_count()
            && self.grid[row..row + 4]
                .iter()
                .map(|row| row[col])
                .collect::<String>()
                == "XMAS"
        {
            total += 1;
        }

        // check b-t
        if row >= 3
            && self.grid[row - 3..row + 1]
                .iter()
                .map(|row| row[col])
                .collect::<String>()
                == "SAMX"
        {
            total += 1;
        }

        return total;
    }

    fn num_of_x_mas(&self, row: usize, col: usize) -> usize {
        if row + 2 < self.row_count()
            && col + 2 < self.col_count()
            && self.grid[row + 1][col + 1] == 'A'
        {
            let top = format!("{}{}", self.grid[row][col], self.grid[row][col + 2]);
            let bottom = format!("{}{}", self.grid[row + 2][col], self.grid[row + 2][col + 2]);

            if top == "MS" && bottom == "MS" {
                return 1;
            }
            if top == "SM" && bottom == "SM" {
                return 1;
            }
            if top == "SS" && bottom == "MM" {
                return 1;
            }
            if top == "MM" && bottom == "SS" {
                return 1;
            }
        }

        return 0;
    }
}

fn part1(input: &str) -> usize {
    let word_search = WordSearch::new(input);
    let mut total = 0;

    for row in 0..word_search.row_count() {
        for col in 0..word_search.col_count() {
            total += word_search.num_of_xmas(row, col);
        }
    }

    return total;
}

fn part2(input: &str) -> usize {
    let word_search = WordSearch::new(input);
    let mut total = 0;

    for row in 0..word_search.row_count() {
        for col in 0..word_search.col_count() {
            total += word_search.num_of_x_mas(row, col);
        }
    }

    return total;
}

fn main() {
    let input = utils::read_input_file(4).expect("failed to open input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn day4_part1() {
        let res = part1(INPUT);
        assert_eq!(res, 18);
    }

    #[test]
    fn day4_part2() {
        let res = part2(INPUT);
        assert_eq!(res, 9);
    }
}
