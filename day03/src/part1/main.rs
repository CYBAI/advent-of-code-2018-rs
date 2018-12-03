#[macro_use]
extern crate lazy_static;

use aoc_utils::read_file;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Claim {
    number: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Claim {
    fn new(number: i32, x: i32, y: i32, width: i32, height: i32) -> Self {
        Claim {
            number,
            x,
            y,
            width,
            height,
        }
    }
}

fn main() {
    lazy_static! {
        pub static ref CLAIM_REGEX: Regex =
            Regex::new(r"\#(?P<n>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)",)
                .unwrap();
    }

    if let Ok(contents) = read_file("./input") {
        let result: i32 = contents
            .lines()
            .filter_map(|line| {
                if let Some(caps) = CLAIM_REGEX.captures(line) {
                    return Some(Claim::new(
                        caps["n"].parse::<i32>().unwrap(),
                        caps["x"].parse::<i32>().unwrap(),
                        caps["y"].parse::<i32>().unwrap(),
                        caps["width"].parse::<i32>().unwrap(),
                        caps["height"].parse::<i32>().unwrap(),
                    ));
                }

                None
            })
            .map(|claim| {
                let mut v = vec![];

                for i in 0..claim.width {
                    for j in 0..claim.height {
                        v.push((claim.x + i, claim.y + j));
                    }
                }

                v
            })
            .flatten()
            .fold(HashMap::new(), |mut acc, coord| {
                {
                    let counter = acc.entry(coord).or_insert(0);
                    *counter += 1;
                }
                acc
            })
            .into_iter()
            .filter(|&(_, v)| v >= 2)
            .collect::<HashMap<(i32, i32), i32>>()
            .keys()
            .len() as i32;

        println!("{:?}", result);
    }
}
