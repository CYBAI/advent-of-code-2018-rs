extern crate aoc_utils;
extern crate regex;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    pub static ref COORDINATE_REGEX: Regex = Regex::new(r"(?P<x>\d+), (?P<y>\d+)").unwrap();
}
