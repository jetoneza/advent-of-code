pub fn execute(input: &str) -> u32 {
    let lines = input.lines();
    lines.map(|line| calculate_arragements(line)).sum()
}

fn calculate_arragements(line: &str) -> u32 {
    let mut it = line.split_whitespace();

    let line = it.next().unwrap();
    let nums: Vec<u32> = it
        .next()
        .unwrap()
        .split(",")
        .filter_map(|token| token.parse::<u32>().ok())
        .collect();

    let mut result = vec![];
    generate_combinations(line, 0, &mut vec![], &mut result);

    let mut count = 0;
    for r in result {
        let row_split = r.split(".").filter(|w| w.len() > 0).collect::<Vec<&str>>();


        if row_split.len() != nums.len() {
            continue;
        }

        let mut is_possible = true;
        for (i, r) in row_split.iter().enumerate() {
            let num = nums[i];
            if r.len() != num as usize {
                is_possible = false;
                break;
            }
        }

        if is_possible {
            count += 1;
        }
    }

    count
}

fn generate_combinations(s: &str, index: usize, current: &mut Vec<char>, result: &mut Vec<String>) {
    if index == s.len() {
        result.push(current.iter().collect());
        return;
    }

    if s.chars().nth(index) == Some('?') {
        current.push('.');
        generate_combinations(s, index + 1, current, result);
        current.pop();

        current.push('#');
        generate_combinations(s, index + 1, current, result);
        current.pop();
    } else {
        current.push(s.chars().nth(index).unwrap());
        generate_combinations(s, index + 1, current, result);
        current.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute(
            "???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1",
        );

        assert_eq!(result, 21);
    }
}
