extern crate aoc_utils;
extern crate regex;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    pub static ref STEP_REGEX: Regex =
        Regex::new(r"Step (?P<start>[A-Z]) must be finished before step (?P<end>[A-Z]) can begin.")
            .unwrap();
}
