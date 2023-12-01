// Advent of Code - Day 1 Solution
// Link: https://adventofcode.com/2023/day/1
fn main() {
    let input = include_str!("./input/input1.txt");
    let output = part_1(input);

    println!("Answer: {}", output);
}

fn part_1(input: &str) -> u32 {
    let lines = input.split_whitespace();
    lines.map(|line| extract_number_value(line)).sum()
}

fn extract_number_value(line: &str) -> u32 {
    let mut iterator = line.chars();

    let mut first_digit = None;
    let mut second_digit = None;

    loop {
        if first_digit.is_some() && second_digit.is_some() {
            break;
        }

        let mut front = None;
        let mut back = None;

        if first_digit.is_none() {
            front = iterator.next();
        }

        if second_digit.is_none() {
            back = iterator.next_back();
        }

        if front.is_none() && back.is_none() {
            break;
        }

        if front.is_some() {
            let front_value = front.unwrap();

            if front_value.is_numeric() && first_digit.is_none() {
                first_digit = front
            }
        }

        if back.is_some() {
            let back_value = back.unwrap();

            if back_value.is_numeric() && second_digit.is_none() {
                second_digit = back
            }
        }
    }

    let value = match (first_digit, second_digit) {
        (Some(f), Some(s)) => {
            format!("{}{}", f, s)
        }
        (Some(f), None) => {
            format!("{}{}", f, f)
        }
        (None, Some(s)) => {
            format!("{}{}", s, s)
        }
        _ => "0".to_owned(),
    };

    value.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part_1(
            "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
            ",
        );
        assert_eq!(result, 142);
    }
}
