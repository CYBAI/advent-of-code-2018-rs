extern crate aoc_utils;

pub fn fold_polymer(mut acc: Vec<char>, c: char) -> Vec<char> {
    if let Some(last_char) = acc.pop() {
        if (last_char.is_lowercase() && c.is_uppercase()
            || last_char.is_uppercase() && c.is_lowercase())
            && (last_char.to_lowercase().to_string() == c.to_lowercase().to_string())
        {
            return acc;
        }

        acc.push(last_char);
    }

    acc.push(c);

    acc
}
