use aoc_utils::read_file;
use day03::Claim;
use day03::CLAIM_REGEX;

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
