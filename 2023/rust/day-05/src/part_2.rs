pub fn execute(input: &str) -> u32 {
    let mut maps = input.split("\n\n");

    let seed_line = maps.next().unwrap();
    let mut seed_pairs = seed_line
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .filter_map(|num| num.parse::<u32>().ok());

    let mut seeds = vec![];

    while let Some(start) = seed_pairs.next() {
        let range = seed_pairs.next().unwrap();
        let end = start + range - 1;

        seeds.push((start, end));
    }

    // TODO: Improve efficiency. This is a brute force solution.
    let mut location = 0;
    loop {
        let mut curr_map = location;
        let maps = maps.clone().collect::<Vec<&str>>();

        for map in maps.iter().rev() {
            let mut map_it = map.lines();
            map_it.next();

            for mapping in map_it {
                let val_map = get_mapping(curr_map, mapping);

                if val_map.is_some() {
                    curr_map = val_map.unwrap();
                    break;
                }
            }
        }

        let mut exists = false;
        for (start, end) in seeds.clone() {
            exists = curr_map >= start && curr_map <= end;

            if exists {
                break;
            }
        }

        if exists {
            break;
        }

        location += 1;
    }

    location
}

fn get_mapping(num: u32, line_map: &str) -> Option<u32> {
    let mut values = line_map.trim().split_whitespace();
    let dest = values.next().unwrap().parse::<u32>().unwrap();
    let source = values.next().unwrap().parse::<u32>().unwrap();
    let range = values.next().unwrap().parse::<u32>().unwrap();

    if num == dest {
        return Some(source);
    }

    if num < dest {
        return None;
    }

    if num <= dest + (range - 1) {
        let diff = num - dest;

        return Some(source + diff);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute(
            "seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4",
        );

        assert_eq!(result, 46);
    }
}
