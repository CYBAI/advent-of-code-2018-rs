extern crate aoc_utils;
extern crate regex;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    static ref COORDINATE_REGEX: Regex = Regex::new(r"(?P<x>\d+), (?P<y>\d+)").unwrap();
}

pub fn parse_coordinates(contents: &str) -> Vec<(i32, i32)> {
    contents
        .split('\n')
        .filter_map(|line| match COORDINATE_REGEX.captures(line) {
            Some(caps) => match (caps["x"].parse::<i32>(), caps["y"].parse::<i32>()) {
                (Ok(x), Ok(y)) => Some((x, y)),
                _ => None,
            },
            None => None,
        })
        .collect::<Vec<(i32, i32)>>()
}

pub fn find_bounds(coordinates: &[(i32, i32)]) -> Option<(i32, i32, i32, i32)> {
    coordinates
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
        })
}

/// https://en.wikipedia.org/wiki/Taxicab_geometry#Formal_definition
pub fn find_manhattan_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}
