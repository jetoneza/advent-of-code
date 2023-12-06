pub fn execute(input: &str) -> u32 {
    let mut lines = input.lines();

    let times = get_values(lines.next().unwrap());
    let distances = get_values(lines.next().unwrap());

    times
        .iter()
        .enumerate()
        .map(|(idx, time)| {
            let distance = distances[idx];

            let mut win_idx = 1;
            for n in 1..*time {
                let calculated_distance = (time - n) * n;

                if calculated_distance > distance {
                    win_idx = n;
                    break;
                }
            }

            (time + 1) - (win_idx * 2)
        })
        .product()
}

fn get_values(line: &str) -> Vec<u32> {
    line.split(":")
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .filter_map(|num| num.parse::<u32>().ok())
        .collect::<Vec<u32>>()
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

        assert_eq!(result, 288);
    }
}
