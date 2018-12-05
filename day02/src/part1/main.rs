use aoc_utils::read_file;
use std::collections::HashMap;

fn calculate(contents: &str) -> (i32, i32) {
    contents
        .lines()
        .map(|line| {
            line.split("")
                .filter(|s| *s != "")
                .fold(HashMap::new(), |mut a, s| {
                    a.entry(s.to_string())
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);

                    a
                })
                .into_iter()
                .filter(|&(_, v)| v == 2 || v == 3)
                .collect()
        })
        .fold((0, 0), |mut acc, h: HashMap<String, i32>| {
            let reversed_key_h: HashMap<i32, String> = h.into_iter().map(|(k, v)| (v, k)).collect();

            if reversed_key_h.get(&2).is_some() {
                acc.0 += 1;
            }

            if reversed_key_h.get(&3).is_some() {
                acc.1 += 1;
            }

            acc
        })
}

fn main() {
    if let Ok(contents) = read_file("./input") {
        let result: (i32, i32) = calculate(&contents);

        println!("Answer: {:?}", result.0 * result.1);
    }
}

#[test]
fn test() {
    assert_eq!(
        calculate("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab".to_string()),
        (4, 3)
    );
}
