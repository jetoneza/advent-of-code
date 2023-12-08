use std::collections::HashMap;

pub fn execute(input: &str) -> usize {
    let mut lines = input.lines();

    let direction: Vec<char> = lines.next().unwrap().chars().collect();

    lines.next();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    // Create mapping
    for line in lines {
        let mut lt = line.split("=");

        let val = lt.next().unwrap().trim();

        let mut numbers = lt
            .next()
            .unwrap()
            .trim()
            .trim_matches(|c| c == ')' || c == '(')
            .split(',');

        let l = numbers.next().unwrap().trim();
        let r = numbers.next().unwrap().trim();

        map.insert(val, (l, r));
    }
    ///////////////////////////////////

    // Traverse the map
    let mut curr = "AAA";
    let mut hop = 0;
    loop {
        let (l, r) = *map.get(curr).unwrap();

        if direction[hop % direction.len()] == 'L' {
            if l == "ZZZ" {
                break;
            }

            curr = l;
        } else {
            if r == "ZZZ" {
                break;
            }

            curr = r;
        }

        hop += 1;
    }
    ///////////////////////////////////

    // Include the first check as 1 hop
    hop + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute(
            "LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)",
        );

        assert_eq!(result, 6);
    }
}
