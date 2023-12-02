pub fn execute(input: &str) -> u32 {
    let lines = input.split_whitespace();
    lines.map(|line| extract_number_value(line)).sum()
}

fn extract_number_value(line: &str) -> u32 {
    let numbers = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut first = None;
    let mut last = None;

    // Evaluate the words.
    for (word, number) in numbers {
        let occurences: Vec<(usize, _)> = line.match_indices(word).collect();

        for (index, _) in occurences {
            if first.is_none() && last.is_none() {
                first = Some((number * 10, index));
                last = Some((number, index));
            }

            if let Some((_, w_idx)) = first {
                if index < w_idx {
                    first = Some((number * 10, index));
                }
            }

            if let Some((_, w_idx)) = last {
                if index > w_idx {
                    last = Some((number, index));
                }
            }
        }
    }

    // Find numeric character for the first digit and check if
    // index is less than the first word.
    for (idx, val) in line.chars().enumerate() {
        if !val.is_numeric() {
            continue;
        }

        if first.is_none() {
            first = Some((val.to_digit(10).unwrap() * 10, 0));
            break;
        }

        let Some((_, i)) = first else {
            continue;
        };

        if idx < i {
            first = Some((val.to_digit(10).unwrap() * 10, 0));
            break;
        }
    }

    // Find numeric character for the last digit and check if
    // index is greater than the last word.
    for (idx, val) in line.chars().enumerate() {
        if !val.is_numeric() {
            continue;
        }

        if last.is_none() {
            last = Some((val.to_digit(10).unwrap(), 0));
            continue;
        }

        let Some((_, i)) = last else {
            continue;
        };

        if idx > i {
            last = Some((val.to_digit(10).unwrap(), 0));
        }
    }

    match (first, last) {
        (Some((f, _)), Some((s, _))) => f + s,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute(
            "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            ",
        );

        assert_eq!(result, 281);
    }
}
