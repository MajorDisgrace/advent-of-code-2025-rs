pub fn solve_part_1(input: &str) -> i32 {
    let banks = input.trim().split('\n');

    let mut count = 0;
    for bank in banks {
        let batteries: Vec<usize> = bank
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .expect("There are non number values in the Input!") as usize
            })
            .collect();

        let mut first_index = 0;
        for i in 1..(batteries.len() - 1) {
            if batteries[first_index] < batteries[i] {
                first_index = i;
            }
        }

        let mut second_index = first_index + 1;
        for i in (second_index + 1)..batteries.len() {
            if batteries[second_index] < batteries[i] {
                second_index = i;
            }
        }

        count += batteries[first_index] * 10 + batteries[second_index];
    }

    return count as i32;
}

pub fn solve_part_2(input: &str) -> usize {
    let banks = input.trim().split('\n');

    let mut count = 0;
    for bank in banks {
        let batteries: Vec<usize> = bank
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .expect("There are non number values in the Input!") as usize
            })
            .collect();

        let mut joltage = 0;
        let mut start_index = 0;
        for i in (0..12).rev() {
            let current_battery = (start_index..(batteries.len() - i))
                .rev()
                .max_by(|x, y| batteries[*x].cmp(&batteries[*y]))
                .expect("There should be at least 12 numbers in every line");

            const TEN: usize = 10;
            joltage += batteries[current_battery] * TEN.pow(i as u32);
            start_index = current_battery + 1;
        }
        count += joltage;
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT), 357);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(TEST_INPUT), 3121910778619);
    }
}
