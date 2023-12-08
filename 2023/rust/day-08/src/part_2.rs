use std::collections::HashMap;

pub fn execute(input: &str) -> usize {
    let mut lines = input.lines();

    let direction: Vec<char> = lines.next().unwrap().chars().collect();

    lines.next();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut start = vec![];

    // Create mapping
    for line in lines {
        let mut lt = line.split("=");

        let val = lt.next().unwrap().trim();

        if val.chars().next_back().unwrap() == 'A' {
            start.push(val);
        }

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

    let mut z_cycles = vec![];

    for curr in start {
        let mut z_cycle = vec![];
        let mut hop = 0;
        let mut first_z = None;
        let mut next = curr;

        loop {
            while hop == 0 || next.chars().last().unwrap() != 'Z' {
                let (l, r) = *map.get(next).unwrap();
                next = if direction[hop % direction.len()] == 'L' {
                    l
                } else {
                    r
                };

                hop += 1;
            }

            z_cycle.push(hop);

            if first_z.is_none() {
                first_z = Some(next);
                hop = 0;
            } else if next == first_z.unwrap() {
                break;
            }
        }

        z_cycles.push(z_cycle);
    }

    let mut nums_lt = z_cycles.iter().map(|z| z[0]).into_iter();
    let mut lcm = nums_lt.next().unwrap();

    for n in nums_lt {
        lcm = (lcm * n) / gcd(lcm, n);
    }

    lcm
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute(
            "LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)",
        );

        assert_eq!(result, 6);
    }
}
