pub fn solve_part_1(input: &str) -> i32 {
    let mut dial = 50;
    let mut passwd = 0;

    for line in input.lines() {
        let direction = line.chars().next().expect("Input file has empty lines!");

        let rotations: i32 = line
            .get(1..)
            .expect("Input contains lines with less than two characters!")
            .parse()
            .expect("Input contains lines that cannot be parsed!");

        dial = match direction {
            'L' => (dial - rotations) % 100,
            'R' => (dial + rotations) % 100,
            _ => panic!("Input contains lines that don't start with L or R!"),
        };

        if dial == 0 {
            passwd += 1;
        }
    }

    return passwd;
}

pub fn solve_part_2(input: &str) -> i32 {
    let mut dial = 50;
    let mut passwd = 0;

    for line in input.lines() {
        let direction = line.chars().next().expect("Input file has empty lines!");

        let rotations: i32 = line
            .get(1..)
            .expect("Input contains lines with less than two characters!")
            .parse()
            .expect("Input contains lines that cannot be parsed!");

        // if direction == 'L' {
        //     if rotations >= dial {
        //         if dial != 0 {
        //             passwd += 1;
        //         }
        //         passwd += (rotations - dial) / 100;
        //     }
        //     dial = (dial - rotations + 100) % 100;
        // } else if direction == 'R' {
        //     if rotations >= (100 - dial) {
        //         if dial != 0 {
        //             passwd += 1;
        //         }
        //         passwd += (rotations - (100 - dial)) / 100;
        //     }
        //     dial = (dial + rotations + 100) % 100;
        // }
        // println!("{} {}", line, dial);
        // println!("{}", passwd)

        for _ in 0..rotations {
            dial = match direction {
                'L' => (dial - 1 + 100) % 100,
                'R' => (dial + 1 + 100) % 100,
                _ => panic!("Input contains lines that don't start with L or R!"),
            };

            if dial == 0 {
                passwd += 1;
            }
        }
    }

    return passwd;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(TEST_INPUT), 6);
    }
}
