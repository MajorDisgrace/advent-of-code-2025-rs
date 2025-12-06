pub fn solve_part_1(input: &str) -> usize {
    let input = input.trim();

    let mut split_input = input.split("\n\n");

    let ranges: Vec<(usize, usize)> = split_input
        .next()
        .expect("The input should contain a list of ranges and a list of ingredients")
        .lines()
        .map(|l| {
            let mut split = l.split('-');

            return (
                split
                    .next()
                    .expect("Every range should be formated like this: 'start-end'")
                    .parse()
                    .expect("The start of each range should be an integer"),
                split
                    .next()
                    .expect("Every range should be formated like this: 'start-end'")
                    .parse()
                    .expect("The end of each range should be an integer"),
            );
        })
        .collect();

    return split_input
        .next()
        .expect("The input shoulc contain a list of available ingredient IDs")
        .lines()
        .map(|l| {
            l.parse::<usize>()
                .expect("The available ingredient IDs should exclusively be numbers!")
        })
        .filter(|n| ranges.iter().any(|t| t.0 <= *n && *n <= t.1))
        .count();
}

pub fn solve_part_2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(TEST_INPUT), 0);
    }
}
