pub fn solve_part_1(input: &str) -> i64 {
    return input
        .split(',')
        .map(|s| {
            let vec: Vec<&str> = s.split('-').collect();
            let start: i64 = (*vec.get(0).expect("Each range should contain a start!"))
                .parse()
                .expect("Input contains Items that are not numbers");

            let end: i64 = (*vec.get(1).expect("Each range should contain an end!"))
                .parse()
                .expect("Input contains Items that are not numbers");

            return (start, end);
        })
        .flat_map(|t| t.0..=t.1)
        .filter(|n| {
            let digits = match n {
                0 => 1,
                _ => n.abs().ilog10() as u32 + 1,
            };
            // println!("{}", digits);
            let pow_of_10 = (10 as i64).pow(digits / 2);
            return match digits % 2 {
                1 => false,
                0 => n / pow_of_10 == n - (n / pow_of_10) * pow_of_10,
                _ => panic!("Something is very wrong!"),
            };
        })
        .sum();
}

pub fn solve_part_2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT), 1227775554);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(TEST_INPUT), 0);
    }
}
