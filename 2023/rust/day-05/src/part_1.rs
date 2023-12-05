pub fn execute(input: &str) -> u32 {
    let mut maps = input.split("\n\n");

    let seed_line = maps.next().unwrap();
    let seeds = seed_line
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .filter_map(|num| num.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let mut closest = None;

    for seed in seeds.iter() {
        let mut curr_map = *seed;
        for map in maps.clone() {
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

        match closest {
            None => {
                closest = Some(curr_map);
            }
            Some(curr) if curr_map < curr => {
                closest = Some(curr_map);
            }
            _ => {}
        }
    }

    closest.unwrap()
}

fn get_mapping(num: u32, line_map: &str) -> Option<u32> {
    let mut values = line_map.trim().split_whitespace();
    let dest = values.next().unwrap().parse::<u32>().unwrap();
    let source = values.next().unwrap().parse::<u32>().unwrap();
    let range = values.next().unwrap().parse::<u32>().unwrap();

    if num == source {
        return Some(dest);
    }

    if num < source {
        return None;
    }

    if num <= source + (range - 1) {
        let diff = num - source;

        return Some(dest + diff);
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

        assert_eq!(result, 35);
    }
}
