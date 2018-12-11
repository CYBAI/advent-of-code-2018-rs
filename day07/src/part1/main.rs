use aoc_utils::read_file;
use day07::STEP_REGEX;
use std::collections::HashMap;

fn traverse_points(
    begin: &char,
    steps: &[(char, char)],
    counter: &mut HashMap<char, (i32, i32)>,
    total_counts: &HashMap<char, (i32, i32)>,
    layer: i32,
    memoized_later: &mut HashMap<char, i32>,
) -> Vec<char> {
    let mut vec = Vec::new();

    // if let Some((c, _)) = memoized_later.clone().iter().filter(|(w, l)| *l == &layer && *w < begin).next() {
    //     vec.push(*c);
    //     memoized_later.remove(c);

    //     check_len += 1;
    // }

    vec.push(*begin);

    println!("Begin: {:?}", (begin, layer));

    for (start, end) in steps.iter().filter(|(start, _)| start == begin) {
        counter
            .entry(*start)
            .and_modify(|(c, _)| *c += 1)
            .or_insert((1, 0));

        counter
            .entry(*end)
            .and_modify(|(_, c)| *c += 1)
            .or_insert((0, 1));

        println!("{:?}", (start, end, layer + 1));

        if let (Some(total_count), Some(current_count)) = (total_counts.get(end), counter.get(end))
        {
            if total_count.1 - current_count.1 != 0 {
                continue;
            }
        }

        let mut next_points = traverse_points(
            end,
            &steps,
            counter,
            &total_counts,
            layer + 1,
            memoized_later,
        );

        // if vec.len() == check_len {
        //     println!("{:?}", (next_points, vec.clone(), layer + 1));
        // } else {
        // }
        vec.append(&mut next_points);
    }

    if vec.len() == 1 {
        println!("{:?}", (vec.clone(), layer));
    }

    vec
}

fn main() {
    if let Ok(contents) = read_file("./input") {
        let mut steps = contents
            .lines()
            .filter_map(|line| {
                if let Some(caps) = STEP_REGEX.captures(line) {
                    return Some((
                        caps["start"].chars().next().unwrap(),
                        caps["end"].chars().next().unwrap(),
                    ));
                }

                None
            })
            .collect::<Vec<(char, char)>>();

        let total_counts: HashMap<char, (i32, i32)> =
            steps.iter().fold(HashMap::new(), |mut acc, (start, end)| {
                acc.entry(*start)
                    .and_modify(|(start_counter, _)| *start_counter += 1)
                    .or_insert((1, 0));

                acc.entry(*end)
                    .and_modify(|(_, end_counter)| *end_counter += 1)
                    .or_insert((0, 1));

                acc
            });

        let mut begin_only = total_counts
            .iter()
            .filter_map(|(c, (_, end))| if *end == 0 { Some(*c) } else { None })
            .collect::<Vec<char>>();

        begin_only.sort();

        println!("{:?}", total_counts.clone());

        steps.sort();

        let mut counter: HashMap<char, (i32, i32)> = HashMap::new();
        let mut memoized_later: HashMap<char, i32> = HashMap::new();

        let mut vec: Vec<char> = Vec::new();
        let layer = 1;

        for begin in begin_only.iter() {
            vec.append(&mut traverse_points(
                begin,
                &steps,
                &mut counter,
                &total_counts,
                layer,
                &mut memoized_later,
            ));
        }

        println!("{:?}", vec.iter().collect::<String>());
        println!("EBICGKQOVMYZJAWRDPXFSUTNLH");
    }
}
