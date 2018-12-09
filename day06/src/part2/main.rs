use aoc_utils::read_file;
use day06::{find_bounds, find_manhattan_distance, parse_coordinates};

fn main() {
    let maximum = 10000;

    if let Ok(contents) = read_file("./input") {
        let coordinates = parse_coordinates(&contents);
        let bounds = find_bounds(&coordinates);

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
