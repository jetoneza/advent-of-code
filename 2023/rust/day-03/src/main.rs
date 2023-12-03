use day_03::{part_1, part_2};

// Advent of Code - Day 2 Solution
// Link: https://adventofcode.com/2023/day/2
fn main() {
    let input = include_str!("./input.txt");

    let part_1_answer = part_1::execute(input);
    let part_2_answer = part_2::execute(input);

    println!("Answer for part 1: {}", part_1_answer);
    println!("Answer for part 2: {}", part_2_answer);
}
