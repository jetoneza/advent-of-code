pub fn execute(input: &str) -> u64 {
    let mut lines = input.lines();

    let time = get_values(lines.next().unwrap());
    let distance = get_values(lines.next().unwrap());

    let mut win_idx = 1;
    for n in 1..time {
        let calculated_distance = (time - n) * n;

        if calculated_distance > distance {
            win_idx = n;
            break;
        }
    }

    (time + 1) - (win_idx * 2)
}

fn get_values(line: &str) -> u64 {
    line.split(":")
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .expect("Should be a number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute(
            "Time:      7  15   30
             Distance:  9  40  200",
        );

        assert_eq!(result, 71503);
    }
}
