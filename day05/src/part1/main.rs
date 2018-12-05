use aoc_utils::read_file;
use day05::fold_polymer;

fn main() {
    if let Ok(contents) = read_file("./input") {
        let result: i32 = contents
            .split("")
            .filter_map(|s| if s == "" { None } else { s.chars().next() })
            .fold(Vec::new(), fold_polymer)
            .len() as i32;

        println!("{:?}", result);
    }
}
