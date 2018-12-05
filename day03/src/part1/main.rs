use aoc_utils::read_file;
use day03::Claim;
use day03::CLAIM_REGEX;
use std::collections::HashMap;

fn main() {
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
            .flat_map(|claim| {
                let mut v = vec![];

                for i in 0..claim.width {
                    for j in 0..claim.height {
                        v.push((claim.x + i, claim.y + j));
                    }
                }

                v
            })
            .fold(HashMap::new(), |mut acc, coord| {
                acc.entry(coord)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);

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
