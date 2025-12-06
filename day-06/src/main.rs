use std::fs;

mod solution;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("There should be a UTF-8 formatted file input.txt in the project root");

    println!("Solution for Part 1: {}", solution::solve_part_1(&input));
    println!("Solution for Part 2: {}", solution::solve_part_2(&input));
}
