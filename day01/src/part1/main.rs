use aoc_utils::read_file;

fn main() {
    if let Ok(contents) = read_file("./input") {
        let result: i32 = contents
            .split('\n')
            .map(|n| n.parse::<i32>().unwrap())
            .sum();

        println!("{:?}", result);
    }
}
