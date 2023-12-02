pub fn execute(input: &str) -> u32 {
    let lines = input.split("\n");
    lines.map(|line| check_game_possibility(line)).sum()
}

fn check_game_possibility(line: &str) -> u32 {
    let mut l_it = line.split(":");

    let game_number = l_it
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|token| token.parse::<u32>().ok())
        .next_back();

    if game_number.is_none() {
        return 0;
    }

    let game_it = l_it.next().unwrap().split(";");

    for game in game_it {
        for cube in game.split(",") {
            let mut cube_it = cube.split_whitespace();
            let value = cube_it.next().unwrap().parse::<u32>().unwrap();
            let color = cube_it.next().unwrap();

            if color == "red" && value > 12
                || color == "green" && value > 13
                || color == "blue" && value > 14
            {
                return 0;
            }
        }
    }

    game_number.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute(
            "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            ",
        );

        assert_eq!(result, 8);
    }
}
