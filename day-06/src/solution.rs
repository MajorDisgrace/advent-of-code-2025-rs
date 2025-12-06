enum Opp {
    Add,
    Mul,
}

pub fn solve_part_1(input: &str) -> usize {
    let input = input.trim();

    // reverse the Iterator for easy acces to the Operators
    let mut lines = input.lines().rev();

    let opps = lines
        .next()
        .expect("Input should contain at least two lines")
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| match s {
            "+" => Opp::Add,
            "*" => Opp::Mul,
            _ => panic!("The last line should only contain spaces '+' and '*'"),
        })
        .collect::<Vec<_>>();

    let mut num_lines = Vec::new();
    for line in lines {
        num_lines.push(
            line.split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    s.parse::<usize>()
                        .expect("The number lines should only contain spaces and numbers")
                })
                .collect::<Vec<_>>(),
        );
    }
    let num_lines = num_lines;

    let mut count = 0;
    for i in 0..opps.len() {
        let opp = opps.get(i).expect("i can not be greater than opps.len()");
        let mut equation_result = match opp {
            Opp::Add => 0,
            Opp::Mul => 1,
        };

        for l in &num_lines {
            let num = l
                .get(i)
                .expect("there should be as many numbers in each line as there are operators");

            match opp {
                Opp::Add => equation_result += num,
                Opp::Mul => equation_result *= num,
            }
        }
        count += equation_result;
    }

    return count;
}

pub fn solve_part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT), 4277556);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(TEST_INPUT), 0);
    }
}
