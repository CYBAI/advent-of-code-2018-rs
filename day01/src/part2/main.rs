use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[allow(unused)]
fn find_num<'a>(
    total: Option<i32>,
    hash: &'a mut HashMap<i32, i32>,
    numbers: &'a Vec<i32>,
    index: Option<i32>,
) -> (
    Option<i32>,
    &'a mut HashMap<i32, i32>,
    &'a Vec<i32>,
    Option<i32>,
) {
    let current_index = match index {
        None => 1,
        Some(n) => n,
    };

    let total = match total {
        Some(n) => n,
        None => numbers[0],
    };

    let current_total = total + numbers[current_index as usize];

    match hash.get(&current_total) {
        Some(_) => (Some(current_total), hash, &numbers, Some(current_index)),
        None => {
            hash.insert(current_total, 0);

            let next_index = if current_index + 1 == numbers.len() as i32 {
                0
            } else {
                current_index + 1
            };

            find_num(Some(current_total), hash, &numbers, Some(next_index))
        }
    }
}

fn loop_version(numbers: &Vec<i32>) {
    let mut result = HashMap::new();

    let mut i = 1;
    let mut num = numbers[0];

    loop {
        num += numbers[i];

        match result.get(&num) {
            Some(_) => {
                println!("{:?}", num);
                break;
            }
            None => {
                result.insert(num, 0);
                i += 1;
            }
        };

        if i == numbers.len() {
            i = 0;
        }
    }
}

fn main() {
    let mut f = File::open("./input").expect("FILE NOT FOUND");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("CONTENT READ ERROR");

    let numbers: Vec<i32> = contents
        .split("\n")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    // First version written with `loop`
    loop_version(&numbers);

    // Due to not support TCO in rust, I can't use recursive version
    // to run with `input` but it works for these sample cases for part 2.
    //
    // let mut hash = HashMap::new();
    // let result = find_num(None, &mut hash, &numbers, None);
    // println!("{:?}", result.0);
}

#[test]
fn test() {
    assert_eq!(
        find_num(None, &mut HashMap::new(), &vec![1, -1], None)
            .0
            .unwrap(),
        0
    );
    assert_eq!(
        find_num(None, &mut HashMap::new(), &vec![3, 3, 4, -2, -4], None)
            .0
            .unwrap(),
        10
    );
    assert_eq!(
        find_num(None, &mut HashMap::new(), &vec![-6, 3, 8, 5, -6], None)
            .0
            .unwrap(),
        5
    );
    assert_eq!(
        find_num(None, &mut HashMap::new(), &vec![7, 7, -2, -7, -4], None)
            .0
            .unwrap(),
        14
    );
}
