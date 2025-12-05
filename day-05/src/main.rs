use std::fs;
mod solution;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("A valid UTF-8 file input.txt should exist in the project root!");

    println!("Solution for Part 1: {}", solution::solve_part_1(&input));
    println!("Solution for Part 2: {}", solution::solve_part_2(&input));
}
