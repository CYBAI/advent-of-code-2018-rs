#[macro_use]
extern crate lazy_static;

use aoc_utils::read_file;
use regex::Regex;

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

fn overlapped(a: &((i32, i32), (i32, i32)), b: &((i32, i32), (i32, i32))) -> bool {
    let a_w = a.0;
    let a_h = a.1;

    let b_w = b.0;
    let b_h = b.1;

    let x_axis_diff = if a_w >= b_w {
        b_w.1 >= a_w.0
    } else {
        a_w.1 >= b_w.0
    };

    let y_axis_diff = if a_h >= b_h {
        b_h.1 >= a_h.0
    } else {
        a_h.1 >= b_h.0
    };

    x_axis_diff && y_axis_diff
}

fn main() {
    lazy_static! {
        pub static ref CLAIM_REGEX: Regex =
            Regex::new(r"\#(?P<n>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)",)
                .unwrap();
    }

    if let Ok(contents) = read_file("./input") {
        let claims: Vec<((i32, i32), (i32, i32))> = contents
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
                (
                    (claim.x, claim.x + claim.width - 1),
                    (claim.y, claim.y + claim.height - 1),
                )
            })
            .collect();

        for i in 0..claims.len() {
            if claims
                .clone()
                .iter()
                .filter(|&c| *c != claims[i])
                .all(|c| !overlapped(&claims[i], c))
            {
                println!("{:?}", i + 1);
                break;
            }
        }
    }
}
