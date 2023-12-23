#[derive(Debug)]
struct Point(i64, i64);

pub fn execute(input: &str) -> i64 {
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

    let h = universe.len();
    let w = universe[0].len();

    let mut row_expansions = vec![];
    let mut column_expansions = vec![];

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

    let mut points = vec![];

    for y in 0..h {
        for x in 0..w {
            if universe[y][x] == '#' {
                points.push(Point(x as i64, y as i64));
            }
        }
    }

    let mut sum = 0;
    for (i, point1) in points.iter().enumerate() {
        for point2 in points.iter().skip(i + 1) {
            let mut x = (point1.0 - point2.0).abs();
            let mut y = (point1.1 - point2.1).abs();

            let x_start = point1.0.min(point2.0);
            let x_end = point1.0.max(point2.0);

            let mut x_count = 0;
            for c in column_expansions.clone().into_iter() {
                if (c as i64) > x_start && (c as i64) < x_end {
                    x_count += 1;
                    x -= 1;
                }
            }

            let y_start = point1.1.min(point2.1);
            let y_end = point1.1.max(point2.1);

            let mut y_count = 0;
            for r in row_expansions.clone().into_iter() {
                if (r as i64) > y_start && (r as i64) < y_end {
                    y_count += 1;
                    y -= 1;
                }
            }

            // Expand the universe
            sum += (x + (x_count * 1_000_000)) + (y + (y_count * 1_000_000));
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
