use std::collections::HashMap;

pub fn execute(input: &str) -> u32 {
    let lines = input.lines();

    let mut five_oak = vec![];
    let mut four_oak = vec![];
    let mut full_house = vec![];
    let mut three_oak = vec![];
    let mut two_pair = vec![];
    let mut one_pair = vec![];
    let mut high_card = vec![];

    let mut card_power = HashMap::new();
    for (idx, c) in [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ]
    .iter()
    .enumerate()
    {
        card_power.insert(c, idx);
    }

    for line in lines {
        let mut l_lt = line.split_whitespace();
        let value = (
            l_lt.next().unwrap(),
            l_lt.next()
                .unwrap()
                .parse::<u32>()
                .expect("should be a number"),
        );

        let mut map = HashMap::new();

        // Calculate cards counts
        let mut j_count = 0;
        for c in value.0.chars() {
            // Count all jacks for substitution
            if c == 'J' {
                j_count += 1;
                continue;
            }

            let count = map.entry(c).or_insert(0);

            *count += 1;
        }
        //////////////////////////////////////////////////////

        // Subtitute Jacks to the highest number of card
        let mut max = (0, None);
        for k in map.keys() {
            let val = map.get(k).unwrap();
            if *val > max.0 {
                max = (*val, Some(k));
            }
        }
        if j_count > 0 && max.1.is_some() {
            let key = max.1.unwrap();

            map.insert(*key, map.get(key).unwrap() + j_count);
        }
        //////////////////////////////////////////////////////

        // Size of each type, <length of map> - <type> {<card_count>, ...}
        // 1 - Five of a kind {5}
        // 2 - Four of a kind {4, 1}
        // 2 - Full house {3, 2}
        // 3 - Three of a kind {3, 1, 1}
        // 3 - Two pair {2, 2, 1}
        // 4 - One pair {2, 1, 1, 1}
        // 5 - High card {1, 1, 1, 1, 1}
        // _ - All jacks {5}
        let size = map.keys().len();
        match size {
            1 => five_oak.push(value),
            4 => one_pair.push(value),
            5 => high_card.push(value),
            2 => {
                for key in map.keys() {
                    let count = map.get(key).unwrap();

                    if *count == 4 || *count == 1 {
                        four_oak.push(value);
                        break;
                    }

                    if *count == 3 || *count == 2 {
                        full_house.push(value);
                        break;
                    }
                }
            }
            3 => {
                for key in map.keys() {
                    let count = map.get(key).unwrap();

                    if *count == 3 {
                        three_oak.push(value);
                        break;
                    }

                    if *count == 2 {
                        two_pair.push(value);
                        break;
                    }
                }
            }
            // All jacks
            _ => five_oak.push(value),
        }
    }

    let mut rank = 1;
    let mut sum = 0;

    // Order cards and calculate points by rank
    for mut cards in [
        high_card, one_pair, two_pair, three_oak, full_house, four_oak, five_oak,
    ] {
        cards.sort_by(|a, b| compare_cards(a, b, &card_power));
        for card in cards {
            sum += card.1 * rank;
            rank += 1;
        }
    }

    sum
}

fn compare_cards(
    a: &(&str, u32),
    b: &(&str, u32),
    card_power: &HashMap<&char, usize>,
) -> std::cmp::Ordering {
    let mut a_val = 0;
    let mut b_val = 0;

    for (idx, a_c) in a.0.chars().enumerate() {
        let b_c = b.0.chars().nth(idx).unwrap();
        if a_c != b_c {
            a_val = *card_power.get(&a_c).unwrap();

            b_val = *card_power.get(&b_c).unwrap();
            break;
        }
    }

    b_val.partial_cmp(&a_val).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute(
            "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483",
        );

        assert_eq!(result, 5905);
    }
}
