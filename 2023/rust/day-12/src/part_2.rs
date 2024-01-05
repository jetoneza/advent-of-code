use std::u64;

pub fn execute(input: &str) -> u64 {
    let lines = input.lines();
    lines.map(|line| calculate_arragements(line)).sum()
}

fn calculate_arragements(line: &str) -> u64 {
    let mut it = line.split_whitespace();

    let unfolded_line = it.next().unwrap();
    let line = &vec![unfolded_line; 5].join("?");

    let unfolded_nums = it.next().unwrap();
    let nums: Vec<u32> = vec![unfolded_nums; 5]
        .join(",")
        .split(",")
        .filter_map(|token| token.parse::<u32>().ok())
        .collect();

    get_combinations_count(line, nums)
}

// Dynamic programming
fn get_combinations_count(s: &String, nums: Vec<u32>) -> u64 {
    let mut target_runs = nums.clone();
    target_runs.push(0);

    let max_run = target_runs.iter().max();
    let line = format!("{}.", s);

    let len = line.len();
    let runs_len = target_runs.len();

    let mut dp = vec![vec![vec![None; *max_run.unwrap() as usize + 1]; runs_len]; len];

    for i in 0..len {
        let x = line.chars().nth(i).unwrap();

        for j in 0..runs_len {
            for k in 0..target_runs[j] + 1 {
                // Base case
                if i == 0 {
                    if j != 0 {
                        dp[i][j][k as usize] = Some(0);
                        continue;
                    }
                    if x == '#' {
                        if k != 1 {
                            dp[i][j][k as usize] = Some(0);
                            continue;
                        }
                        dp[i][j][k as usize] = Some(1);
                        continue;
                    }
                    if x == '.' {
                        if k != 0 {
                            dp[i][j][k as usize] = Some(0);
                            continue;
                        }
                        dp[i][j][k as usize] = Some(1);
                        continue;
                    }
                    if x == '?' {
                        if k != 0 && k != 1 {
                            dp[i][j][k as usize] = Some(0);
                            continue;
                        }
                        dp[i][j][k as usize] = Some(1);
                        continue;
                    }
                }

                let mut dots = 0;
                if k != 0 {
                    dots = 0;
                } else if j > 0 {
                    assert_eq!(k, 0);
                    dots = dp[i - 1][j - 1][target_runs[j - 1] as usize].unwrap()
                        + dp[i - 1][j][0].unwrap();
                } else {
                    // i>0, j=0, k=0.
                    // Only way to do this is if every ? is a .
                    dots = if line[..i].matches('#').count() == 0 {
                        1
                    } else {
                        0
                    };
                }

                let mut pounds = 0;
                if k == 0 {
                    pounds = 0;
                } else {
                    // Newest set
                    pounds = dp[i - 1][j][k as usize - 1].unwrap();
                }

                if x == '.' {
                    dp[i][j][k as usize] = Some(dots);
                } else if x == '#' {
                    dp[i][j][k as usize] = Some(pounds);
                } else {
                    dp[i][j][k as usize] = Some(dots + pounds);
                }
            }
        }
    }

    dp[len - 1][runs_len - 1][0].unwrap()
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

        assert_eq!(result, 525152);
    }
}
