use aoc_utils::read_file;
use day04::FIND_GUARD_REGEX;
use day04::{Action, Guard, Time};
use std::collections::HashMap;

fn main() {
    if let Ok(contents) = read_file("./input") {
        let mut guards: Vec<Guard> = contents
            .lines()
            .filter_map(|line| {
                if let Some(caps) = FIND_GUARD_REGEX.captures(line) {
                    let mut time = Time::new(
                        caps["year"].parse::<i32>().unwrap(),
                        caps["month"].parse::<i32>().unwrap(),
                        caps["day"].parse::<i32>().unwrap(),
                        caps["hour"].parse::<i32>().unwrap(),
                        caps["minute"].parse::<i32>().unwrap(),
                    );

                    if time.get_hour() != 0 {
                        time.set_to_next_day();
                    }

                    return Some(Guard::new(
                        time,
                        caps.name("guard_id")
                            .map_or(None, |id| match id.as_str().parse::<i32>() {
                                Ok(id) => Some(id),
                                Err(_) => None,
                            }),
                        Action::parse_str_to_action(&caps["action"]).unwrap(),
                    ));
                }

                None
            })
            .collect();

        guards.sort();

        let guard_sleepy_table: HashMap<i32, HashMap<String, Vec<i32>>> = guards
            .into_iter()
            .fold((0, (Action::Begin, 0), HashMap::new()), |mut acc, guard| {
                if let Some(id) = guard.get_guard_id() {
                    acc.0 = id;
                }

                let time = guard.get_time();

                {
                    let cache = acc.2.entry(acc.0).or_insert(HashMap::new());

                    let minutes_vec = cache.entry(time.get_date_string()).or_insert(vec![]);

                    let previous_action = acc.1;

                    if let (Action::Asleep, Action::Awake) = (previous_action.0, guard.get_action())
                    {
                        for n in previous_action.1..time.get_minute() {
                            minutes_vec.push(n);
                        }
                    }
                }

                acc.1 = (guard.get_action(), time.get_minute());

                acc
            })
            .2;

        let laziest_guard: (i32, Vec<i32>) = guard_sleepy_table
            .into_iter()
            .map(|(id, x)| {
                (
                    id,
                    x.into_iter()
                        .filter(|(_, v)| !v.is_empty())
                        .fold(vec![], |acc, (_, v)| [acc, v].concat()),
                )
            })
            .max_by(|(_, x), (_, y)| x.len().cmp(&y.len()))
            .unwrap();

        let good_timing: (i32, i32) = laziest_guard
            .1
            .clone()
            .into_iter()
            .fold(HashMap::new(), |mut acc, n| {
                {
                    let counter = acc.entry(n).or_insert(0);

                    *counter += 1;
                }

                acc
            })
            .into_iter()
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .unwrap();

        println!("{:?}", laziest_guard.0 * good_timing.0);
    }
}
