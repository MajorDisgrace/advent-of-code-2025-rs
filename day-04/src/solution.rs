pub fn solve_part_1(input: &str) -> i32 {
    let input = input.trim();

    0
}

pub fn solve_part_2(input: &str) -> i32 {
    0
}

fn str_to_vec_matrix(s: &str) -> Vec<Vec<u8>> {}

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
        assert_eq!(solve_part_1(TEST_INPUT), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(TEST_INPUT), 0);
    }
}
