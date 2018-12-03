extern crate aoc_utils;
extern crate regex;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    pub static ref CLAIM_REGEX: Regex =
        Regex::new(r"\#(?P<n>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)",)
            .unwrap();
}

#[derive(Debug)]
pub struct Claim {
    pub number: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Claim {
    pub fn new(number: i32, x: i32, y: i32, width: i32, height: i32) -> Self {
        Claim {
            number,
            x,
            y,
            width,
            height,
        }
    }
}
