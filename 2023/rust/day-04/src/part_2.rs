use std::collections::HashMap;

pub fn execute(input: &str) -> u32 {
    let lines = input.lines();

    let mut map = HashMap::new();

    // TODO: Improve performance
    for (idx, val) in lines.map(|line| calculate_points(line)).enumerate() {
        let default_val = if val == 0 { 0 } else { 1 };

        let curr_val = map
            .entry(idx)
            .and_modify(|counter| *counter += 1)
            .or_insert(default_val)
            .to_owned();

        let mut i = 0;

        while i < curr_val {
            for i in (idx + 1)..(idx + 1 + val as usize) {
                let game = map.entry(i).or_insert(0);

                if *game < 1 {
                    *game = 1;
                    continue;
                }

                *game += 1;
            }

            i += 1;
        }
    }

    map.into_iter()
        .map(|(_, val)| {
            if val == 0 {
                return 1;
            }

            val
        })
        .sum::<u32>()
}

fn calculate_points(line: &str) -> u32 {
    let game = line.split(":").last().unwrap().trim();

    let mut game_it = game.split("|");

    let winning_numbers = get_numbers(game_it.next().unwrap());
    let numbers = get_numbers(game_it.next().unwrap());

    let mut winnings = 0;

    for n in numbers.iter() {
        if winning_numbers.contains(n) {
            winnings += 1;
        }
    }

    winnings
}

fn get_numbers(entries: &str) -> Vec<u32> {
    entries
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
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );

        assert_eq!(result, 30);
    }
}
