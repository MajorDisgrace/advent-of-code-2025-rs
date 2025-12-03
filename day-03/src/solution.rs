pub fn solve_part_1(input: &str) -> i32 {
    let banks = input.trim().split('\n');

    for bank in banks {
        let batteries: Vec<u8> = bank
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .expect("There are non Number values in the Input!") as u8
            })
            .collect();

        let mut first_battery = batteries[0];
        for i in 1..(batteries.len() - 1) {
            if first_battery < batteries[i] {
                first_battery = batteries[i];
            }
        }
    }
    0
}

pub fn solve_part_2(input: &str) -> i32 {
    0
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
        assert_eq!(solve_part_2(TEST_INPUT), 0);
    }
}
