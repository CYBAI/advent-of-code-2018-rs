use aoc_utils::read_file;
use day06::{find_bounds, find_manhattan_distance, parse_coordinates};
use std::collections::HashMap;

fn main() {
    if let Ok(contents) = read_file("./input") {
        let coordinates = parse_coordinates(&contents);
        let bounds = find_bounds(&coordinates);

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
