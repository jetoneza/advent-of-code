pub fn execute(input: &str) -> i32 {
    let lines = input.lines();

    let mut sum = 0;
    for line in lines {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        let mut diffs: Vec<i32> = vec![];

        let mut next_nums = nums.clone();
        loop {
            let mut curr = None;
            let mut next_diff = vec![];
            for num in next_nums.iter().rev() {
                if curr.is_none() {
                    curr = Some(num);
                    diffs.push(*num);
                    continue;
                }

                let diff = curr.unwrap() - num;

                next_diff.push(diff);
                curr = Some(num);
            }

            if let Some(last) = diffs.last() {
                if *last == 0 {
                    break;
                }
            }

            next_diff.reverse();
            next_nums = next_diff;
        }

        sum += diffs.iter().sum::<i32>();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute(
            "0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45",
        );

        assert_eq!(result, 0);
    }
}
