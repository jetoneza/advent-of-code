use day_04::part_1;

// Advent of Code - Day 4 Solution
// Link: https://adventofcode.com/2023/day/4
fn main() {
    let input = include_str!("./input.txt");

    let part_1_answer = part_1::execute(input);

    println!("Answer for part 1: {}", part_1_answer);
}
