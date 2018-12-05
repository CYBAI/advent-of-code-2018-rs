use aoc_utils::read_file;

fn main() {
    if let Ok(contents) = read_file("./input") {
        let result: i32 = contents
            .split("")
            .filter_map(|s| if s == "" { None } else { s.chars().next() })
            .fold(Vec::new(), |mut acc: Vec<char>, c| {
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
            })
            .len() as i32;

        println!("{:?}", result);
    }
}
