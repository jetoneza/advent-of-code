pub fn execute(input: &str) -> u32 {
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
                first_digit = Some(front_value.to_digit(10).unwrap() * 10);
            }
        }

        if back.is_some() {
            let back_value = back.unwrap();

            if back_value.is_numeric() && second_digit.is_none() {
                second_digit = Some(back_value.to_digit(10).unwrap());
            }
        }
    }

    match (first_digit, second_digit) {
        (Some(f), Some(s)) => f + s,
        (Some(f), None) => f + (f / 10),
        (None, Some(s)) => (s * 10) + s,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let part_1_result = execute(
            "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
            ",
        );

        assert_eq!(part_1_result, 142);
    }
}
