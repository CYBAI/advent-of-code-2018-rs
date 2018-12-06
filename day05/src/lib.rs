extern crate aoc_utils;

pub fn fold_polymer(mut acc: Vec<char>, c: char) -> Vec<char> {
    if let Some(last_char) = acc.pop() {
        if last_char.eq_ignore_ascii_case(&c) && last_char != c {
            return acc;
        }

        acc.push(last_char);
    }

    acc.push(c);

    acc
}
