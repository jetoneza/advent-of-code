use std::u32;

pub fn execute(input: &str) -> u32 {
    let lines = input.split("\n");

    // Line number
    let mut l_num = 1;

    // List of detected numbers
    // tuple structure: (line number, number value, start index, end index)
    let mut nums = Vec::new();
    // List of detected symbols
    // tuple structure: (line number, index)
    let mut symbols = Vec::new();

    for line in lines {
        if line == "" {
            continue;
        }

        let mut curr_num = None;
        let mut curr_idx = 1;

        for ch in line.chars() {
            if ch.is_numeric() {
                // Check for a number and record its start and ending.
                if curr_num.is_none() {
                    curr_num = Some((l_num, ch.to_string(), curr_idx, curr_idx));
                } else {
                    let (ln, num, start, _) = curr_num.unwrap();
                    let num_val = format!("{num}{ch}");

                    curr_num = Some((ln, num_val, start, curr_idx));
                }
            } else {
                if ch == '*' {
                    // Detect gear symbol and record the placement.
                    symbols.push((l_num, curr_idx));
                }

                if curr_num.is_some() {
                    nums.push(curr_num);

                    curr_num = None;
                }
            }

            curr_idx += 1;

            // Detect if there is a number at the end of the line
            if curr_idx == line.len() + 1 && curr_num.is_some() {
                nums.push(curr_num);

                curr_num = None;
            }
        }

        l_num += 1;
    }

    symbols
        .into_iter()
        .filter_map(|symbol| {
            let (s_ln, s_idx) = symbol;

            let mut connections: Vec<u32> = Vec::new();

            // Detect the gear connections.
            for num in nums.clone() {
                let (ln, val, start, end) = num.unwrap();

                if (s_ln >= ln - 1 && s_ln <= ln + 1) && (s_idx >= start - 1 && s_idx <= end + 1) {
                    let num_val = val.parse::<u32>().unwrap();

                    connections.push(num_val);
                }
            }

            if connections.len() == 2 {
                return Some(connections.iter().product::<u32>());
            }

            None
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute(
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            ",
        );

        assert_eq!(result, 467835);
    }
}
