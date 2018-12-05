use aoc_utils::read_file;

fn find_one_diff(a: &str, b: &str) -> Option<String> {
    let diff = a.len() - a.chars().zip(b.chars()).filter(|&(a, b)| a == b).count();

    if diff == 1 {
        return Some(
            a.chars()
                .zip(b.chars())
                .filter_map(|(a, b)| {
                    if a == b {
                        Some(a)
                    } else {
                        None
                    }
                })
                .collect(),
        );
    }

    None
}

fn main() {
    if let Ok(contents) = read_file("./input") {
        let inputs: Vec<String> = contents
            .lines()
            .map(|line| line.split("").collect())
            .collect();

        for i in 1..inputs.len() {
            let (left, right) = inputs.split_at(i);

            if let Some(s) = right
                .iter()
                .map(|s| find_one_diff(&left[i - 1], s))
                .find(|x| x.is_some())
            {
                println!("{:?}", s.unwrap());

                break;
            }
        }
    }
}
