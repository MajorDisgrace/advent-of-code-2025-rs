pub fn solve_part_1(input: &str) -> i64 {
    return input
        .trim()
        .split(',')
        .flat_map(|s| str_to_range(s))
        .filter(|n| is_symmetric(*n))
        .sum();
}

pub fn solve_part_2(input: &str) -> i64 {
    return input
        .trim()
        .split(',')
        .flat_map(|s| str_to_range(s))
        .filter(|n| is_repeating(*n))
        .sum();
}

fn str_to_range(input: &str) -> core::ops::RangeInclusive<i64> {
    let mut boundaries = input.split('-');

    let start: i64 = boundaries
        .next()
        .expect("Each range should contain a start!")
        .parse()
        .expect("Input contains Items that are not numbers");

    let end: i64 = boundaries
        .next()
        .expect("Each range should contain an end!")
        .parse()
        .expect("Input contains Items that are not numbers");

    return start..=end;
}

fn is_repeating(n: i64) -> bool {
    let s = n.to_string();
    let len = s.len();

    for sub_len in 1..=(len / 2) {
        if len % sub_len != 0 {
            continue;
        }

        let sub_str = &s[..sub_len];

        if s == sub_str.repeat(len / sub_len) {
            return true;
        }
    }

    false
}

fn is_symmetric(n: i64) -> bool {
    let digits = match n {
        0 => 1,
        _ => n.abs().ilog10() as u32 + 1,
    };

    if digits % 2 == 1 {
        return false;
    }

    let center = (10 as i64).pow(digits / 2);
    return n / center == n - (n / center) * center;
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
        assert_eq!(solve_part_2(TEST_INPUT), 4174379265);
    }
}
