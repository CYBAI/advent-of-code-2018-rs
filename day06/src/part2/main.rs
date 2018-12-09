use aoc_utils::read_file;
use day06::COORDINATE_REGEX;
use day06::find_manhattan_distance;

fn main() {
    let maximum = 10000;

    if let Ok(contents) = read_file("./input") {
        let coordinates = contents
            .split('\n')
            .filter_map(|line| match COORDINATE_REGEX.captures(line) {
                Some(caps) => match (caps["x"].parse::<i32>(), caps["y"].parse::<i32>()) {
                    (Ok(x), Ok(y)) => Some((x, y)),
                    _ => None,
                },
                None => None,
            })
            .collect::<Vec<(i32, i32)>>();

        let bounds = coordinates
            .iter()
            .fold(None, |bounds, coordinate| match bounds {
                None => Some((coordinate.0, coordinate.1, coordinate.0, coordinate.1)),
                Some(bounds) => {
                    let min_x = bounds.0.min(coordinate.0);
                    let min_y = bounds.1.min(coordinate.1);
                    let max_x = bounds.2.max(coordinate.0);
                    let max_y = bounds.3.max(coordinate.1);
                    Some((min_x, min_y, max_x, max_y))
                }
            });

        let (min_x, min_y, max_x, max_y) = match bounds {
            Some(bounds) => bounds,
            None => return,
        };

        let mut counter = 0;

        for i in min_x..=max_x {
            for j in min_y..=max_y {
                let distances = coordinates
                    .iter()
                    .map(|c| find_manhattan_distance(*c, (i, j)))
                    .collect::<Vec<i32>>();

                let sum: i32 = distances.iter().sum();

                if sum < maximum {
                    counter += 1;
                }
            }
        }

        println!("{:?}", counter);
    }
}
