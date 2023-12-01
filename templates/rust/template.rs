// Advent of Code - Day 1 Solution
// Link: https://adventofcode.com/2023/day/1
fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
}

fn part_1(input: &str) -> String {
    "sample".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part_1("");
        assert_eq!(result, "sample".to_string());
    }
}
