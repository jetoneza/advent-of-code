#[derive(Debug)]
struct Point(i32, i32);

pub fn execute(input: &str) -> i32 {
    let lines = input.lines();

    let mut universe = vec![];

    // Extract the original universe
    for line in lines {
        let mut columns = vec![];

        for c in line.chars() {
            columns.push(c);
        }

        universe.push(columns);
    }

    let mut h = universe.len();
    let mut w = universe[0].len();

    let mut row_expansions = vec![];
    let mut column_expansions = vec![];

    // Expand the universe
    for y in 0..h {
        let mut is_clear = true;
        for x in 0..w {
            if universe[y][x] == '#' {
                is_clear = false;
            }
        }

        if is_clear {
            row_expansions.push(y);
        }
    }

    for x in 0..w {
        let mut is_clear = true;
        for y in 0..h {
            if universe[y][x] == '#' {
                is_clear = false;
            }
        }

        if is_clear {
            column_expansions.push(x);
        }
    }

    for (i, x) in column_expansions.iter().enumerate() {
        for y in 0..h {
            universe[y].insert(x + i, '.')
        }
    }

    w += column_expansions.len();

    for (i, y) in row_expansions.iter().enumerate() {
        universe.insert(y + i, vec!['.'; w])
    }

    h += row_expansions.len();

    let mut points = vec![];

    for y in 0..h {
        for x in 0..w {
            if universe[y][x] == '#' {
                points.push(Point(x as i32, y as i32));
            }
        }
    }

    let mut sum = 0;
    for (i, point1) in points.iter().enumerate() {
        for point2 in points.iter().skip(i + 1) {
            sum += (point1.0 - point2.0).abs() + (point1.1 - point2.1).abs();
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute(
            "...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....",
        );

        assert_eq!(result, 374);
    }
}
