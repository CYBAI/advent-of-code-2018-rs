use aoc_utils::read_file;
use day06::COORDINATE_REGEX;
use std::collections::HashMap;

/// https://en.wikipedia.org/wiki/Taxicab_geometry#Formal_definition
fn find_manhattan_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn main() {
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

        let mut hash: HashMap<i32, (i32, bool)> = HashMap::new();

        for i in min_x..=max_x {
            for j in min_y..=max_y {
                let distances = coordinates
                    .iter()
                    .map(|c| find_manhattan_distance(*c, (i, j)))
                    .collect::<Vec<i32>>();

                let min = distances.iter().min().unwrap();

                let mins = distances
                    .iter()
                    .enumerate()
                    .filter_map(|(i, n)| if n == min { Some(i as i32) } else { None })
                    .collect::<Vec<i32>>();

                if mins.len() == 1 {
                    let on_x_bound = i == min_x || i == max_x;
                    let on_y_bound = j == min_y || j == max_y;

                    let (counter, on_bound) = hash.entry(mins[0]).or_insert((0, false));

                    if !*on_bound && (on_x_bound || on_y_bound) {
                        *on_bound = true;
                    }

                    *counter += 1;
                }
            }
        }

        let result = hash
            .iter()
            .filter(|(_, (_, on_bound))| !on_bound)
            .max_by_key(|(_, (counter, _))| counter)
            .unwrap();

        println!("{:?}", (result.1).0);
    }
}
