use aoc_utils::read_file;
use day06::{find_bounds, find_manhattan_distance, parse_coordinates};
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    if let Ok(contents) = read_file("./input") {
        let coordinates = parse_coordinates(&contents);
        let bounds = find_bounds(&coordinates);

        let (min_x, min_y, max_x, max_y) = match bounds {
            Some(bounds) => bounds,
            None => return,
        };

        let result = (min_x..=max_x)
            .cartesian_product(min_y..=max_y)
            .map(|(x, y)| {
                (
                    (x, y),
                    coordinates
                        .iter()
                        .map(|c| find_manhattan_distance(*c, (x, y)))
                        .collect::<Vec<i32>>(),
                )
            })
            .filter_map(|(coordinate, distances)| {
                let min = distances.iter().min().unwrap();

                let mins = distances
                    .iter()
                    .enumerate()
                    .filter_map(|(i, n)| if n == min { Some(i as i32) } else { None })
                    .collect::<Vec<i32>>();

                if mins.len() == 1 {
                    Some((coordinate, Some(mins[0])))
                } else {
                    Some((coordinate, None))
                }
            })
            .fold(HashMap::new(), |mut acc, ((x, y), min)| {
                if let Some(min) = min {
                    let on_x_bound = x == min_x || x == max_x;
                    let on_y_bound = y == min_y || y == max_y;

                    let (counter, on_bound) = acc.entry(min).or_insert((0, false));

                    if !*on_bound && (on_x_bound || on_y_bound) {
                        *on_bound = true;
                    }

                    *counter += 1;
                }

                acc
            })
            .iter()
            .filter_map(|(_, (counter, on_bound))| if !on_bound { Some(*counter) } else { None })
            .max()
            .unwrap();

        println!("{:?}", result);
    }
}
