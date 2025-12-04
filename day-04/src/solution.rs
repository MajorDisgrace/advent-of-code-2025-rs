struct Matrix {
    data: Vec<u8>,
    cols: usize,
}

impl Matrix {
    fn get(&self, row: usize, col: usize) -> Option<u8> {
        if col >= self.cols() {
            return None;
        }
        if row >= self.rows() {
            return None;
        }

        let result = *self.data.get(row * self.cols + col)?;
        return Some(result);
    }

    fn rows(&self) -> usize {
        self.data.len() / self.cols()
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut u8> {
        if col >= self.cols() {
            return None;
        }
        if row >= self.rows() {
            return None;
        }

        let result = self.data.get_mut(row * self.cols + col)?;
        return Some(result);
    }
}

pub fn solve_part_1(input: &str) -> i32 {
    let input = input.trim();
    let matrix = str_to_matrix(input);

    let mut count = 0;
    for row in 0..matrix.rows() {
        for col in 0..matrix.cols() {
            if matrix.get(row, col) == Some(1)
                && count_adj_paper(&matrix, row as isize, col as isize) < 4
            {
                count += 1;
            }
        }
    }

    return count;
}

pub fn solve_part_2(input: &str) -> i32 {
    let input = input.trim();
    let mut matrix = str_to_matrix(input);

    let mut count = 0;
    let mut last_count = -1;
    while count != last_count {
        last_count = count;
        for row in 0..matrix.rows() {
            for col in 0..matrix.cols() {
                if matrix.get(row, col) == Some(1)
                    && count_adj_paper(&matrix, row as isize, col as isize) < 4
                {
                    let val = matrix
                        .get_mut(row, col)
                        .expect("Something went very wrong!");
                    *val = 0;
                    count += 1;
                }
            }
        }
    }

    return count;
}

fn count_adj_paper(matrix: &Matrix, row: isize, col: isize) -> u8 {
    let mut count = 0;

    for r in (row - 1)..=(row + 1) {
        for c in (col - 1)..=(col + 1) {
            if c == col && r == row {
                //only want adjacent cells
                continue;
            }
            if c < 0 || r < 0 {
                continue;
            }

            count += match matrix.get(r as usize, c as usize) {
                Some(n) => n,
                None => 0,
            }
        }
    }

    return count;
}

fn str_to_matrix(s: &str) -> Matrix {
    let cols = s
        .lines()
        .next()
        .expect("Input should contain at least one line!")
        .chars()
        .count();

    let data: Vec<u8> = s
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| match c {
            '@' => 1 as u8,
            '.' => 0 as u8,
            _ => panic!("The input should only contain chars of type '@' and '.'"),
        })
        .collect();

    return Matrix {
        data: data,
        cols: cols,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(TEST_INPUT), 43);
    }
}
