use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("./input").expect("FILE NOT FOUND");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("CONTENT READ ERROR");

    let result: i32 = contents
        .split("\n")
        .map(|n| n.parse::<i32>().unwrap())
        .sum();

    println!("{:?}", result);
}
