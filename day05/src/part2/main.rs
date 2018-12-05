use aoc_utils::read_file;
use day05::fold_polymer;
use std::collections::HashSet;

fn main() {
    if let Ok(contents) = read_file("./input") {
        let chars: Vec<char> = contents
            .split("")
            .filter_map(|s| if s == "" { None } else { s.chars().next() })
            .collect();

        let result = chars
            .clone()
            .into_iter()
            .fold(HashSet::new(), |mut set: HashSet<String>, c| {
                set.insert(c.to_lowercase().to_string());

                set
            })
            .iter()
            .map(|s| {
                chars
                    .clone()
                    .into_iter()
                    .filter(|c| &c.to_lowercase().to_string() != s)
                    .fold(Vec::new(), fold_polymer)
                    .len()
            })
            .min()
            .unwrap_or(0);

        println!("{:?}", result);
    }
}
