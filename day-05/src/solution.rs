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

pub fn solve_part_2(input: &str) -> usize {
    let mut ranges = input
        .trim()
        .split("\n\n")
        .next()
        .expect("The input should contain a list of ranges and a list of ingredients")
        .lines()
        .map(|l| {
            let mut split = l.split('-');

            return (
                split
                    .next()
                    .expect("Every range should be formated like this: 'start-end'")
                    .parse::<usize>()
                    .expect("The start of each range should be an integer"),
                split
                    .next()
                    .expect("Every range should be formated like this: 'start-end'")
                    .parse::<usize>()
                    .expect("The end of each range should be an integer"),
            );
        })
        .collect::<Vec<_>>();

    ranges.sort_unstable_by(|(a1, _), (b1, _)| a1.cmp(b1));

    let mut unique_ranges = Vec::new();
    let mut previous_end = 0;
    for range in ranges {
        if unique_ranges.len() == 0 {
            unique_ranges.push(range);
            previous_end = range.1;
            continue;
        }

        if range.0 <= previous_end && previous_end < range.1 {
            let t = unique_ranges.pop().expect(
                "unique_ranges is initialized to contain one tuple and no item is ever removed",
            );
            unique_ranges.push((t.0, range.1));
            previous_end = range.1;
            continue;
        } else if range.0 <= previous_end {
            continue;
        }

        unique_ranges.push(range);
        previous_end = range.1;
    }

    return unique_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum();
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
        assert_eq!(solve_part_2(TEST_INPUT), 14);
    }
}
