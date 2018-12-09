use aoc_utils::read_file;
use day06::{find_bounds, find_manhattan_distance, parse_coordinates};
use itertools::Itertools;

fn main() {
    let maximum = 10000;

    if let Ok(contents) = read_file("./input") {
        let coordinates = parse_coordinates(&contents);
        let bounds = find_bounds(&coordinates);

        let (min_x, min_y, max_x, max_y) = match bounds {
            Some(bounds) => bounds,
            None => return,
        };

        let result: i32 = (min_x..=max_x)
            .cartesian_product(min_y..=max_y)
            .filter(|(x, y)| {
                coordinates
                    .iter()
                    .map(|c| find_manhattan_distance(*c, (*x, *y)))
                    .sum::<i32>()
                    < maximum
            })
            .count() as i32;

        println!("{:?}", result);
    }
}
