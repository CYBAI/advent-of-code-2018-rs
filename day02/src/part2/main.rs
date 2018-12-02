use std::fs::File;
use std::io::prelude::*;

fn find_one_diff(a: &str, b: &str) -> Option<String> {
    let diff = a.len() - a.chars().zip(b.chars()).filter(|&(a, b)| a == b).count();

    if diff == 1 {
        return Some(a.chars().zip(b.chars()).filter(|&(a, b)| a == b).map(|(v, _)| v).collect());
    }

    None
}

fn main() {
    let mut f = File::open("./input").expect("FILE NOT FOUND");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("CONTENT READ ERROR");

    let inputs: Vec<String> = contents
        .lines()
        .map(|line| {
            line.split("").collect()
        })
        .collect();

    for i in 1..inputs.len() {
        let (left, right) = inputs.split_at(i);

        if let Some(s) = right.iter().map(|s| find_one_diff(&left[i - 1], s)).find(|x| x.is_some()) {
            println!("{:?}", s.unwrap());

            break;
        }
    }
}