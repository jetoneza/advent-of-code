#[derive(Debug)]
struct Point(i32, i32);

pub fn execute(input: &str) -> u32 {
    let lines = input.lines();

    let mut maze = vec![];
    let mut start = None;

    for (y, line) in lines.enumerate() {
        let mut columns = vec![];

        for (x, c) in line.chars().enumerate() {
            columns.push(c);

            if c == 'S' {
                start = Some(Point(x as i32, y as i32));
            }
        }

        maze.push(columns);
    }

    let steps = walk(&maze, &start.unwrap(), None, 0);

    steps / 2
}

fn walk(maze: &Vec<Vec<char>>, curr: &Point, last: Option<&Point>, steps: u32) -> u32 {
    let x = curr.0;
    let y = curr.1;

    if x < 0 || x >= maze[0].len() as i32 || y < 0 || y >= maze.len() as i32 {
        return 0;
    }

    let destination = maze[y as usize][x as usize];

    if destination == 'S' && steps > 0 {
        return steps;
    }

    if destination == '.' {
        return 0;
    }

    let destinations = vec![Point(0, -1), Point(1, 0), Point(0, 1), Point(-1, 0)];

    for point in destinations {
        let x = curr.0 + point.0;
        let y = curr.1 + point.1;

        if let Some(last_point) = last {
            if x == last_point.0 && y == last_point.1 {
                continue;
            }
        }

        if x < 0 || x >= maze[0].len() as i32 || y < 0 || y >= maze.len() as i32 {
            continue;
        }

        if !check_path(&maze, &curr, &point) {
            continue;
        }

        let s = walk(&maze, &Point(x, y), Some(&curr), steps + 1);

        if s > 0 {
            return s;
        }
    }

    0
}

fn check_path(maze: &Vec<Vec<char>>, from: &Point, to: &Point) -> bool {
    let from_char = maze[from.1 as usize][from.0 as usize];
    let to_char = maze[(from.1 + to.1) as usize][(from.0 + to.0) as usize];

    let contains = match (from_char, to.0, to.1) {
        ('S', 0, -1) | ('|', 0, -1) | ('L', 0, -1) | ('J', 0, -1) => {
            ['|', '7', 'F', 'S'].contains(&to_char)
        }
        ('S', 0, 1) | ('|', 0, 1) | ('7', 0, 1) | ('F', 0, 1) => {
            ['|', 'L', 'J', 'S'].contains(&to_char)
        }
        ('S', 1, 0) | ('-', 1, 0) | ('L', 1, 0) | ('F', 1, 0) => {
            ['-', '7', 'J', 'S'].contains(&to_char)
        }
        ('S', -1, 0) | ('-', -1, 0) | ('J', -1, 0) | ('7', -1, 0) => {
            ['-', 'L', 'F', 'S'].contains(&to_char)
        }
        _ => false,
    };

    contains
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute(
            "..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...",
        );

        assert_eq!(result, 0);
    }
}
