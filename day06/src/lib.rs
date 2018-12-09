extern crate aoc_utils;
extern crate regex;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    pub static ref COORDINATE_REGEX: Regex = Regex::new(r"(?P<x>\d+), (?P<y>\d+)").unwrap();
}

/// https://en.wikipedia.org/wiki/Taxicab_geometry#Formal_definition
pub fn find_manhattan_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}
