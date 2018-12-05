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
                            .and_then(|id| match id.as_str().parse::<i32>() {
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
                    let cache = acc.2.entry(acc.0).or_insert_with(HashMap::new);

                    let minutes_vec = cache.entry(time.get_date_string()).or_insert_with(Vec::new);

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

        let sleepiest_guard: (i32, (i32, i32)) = guard_sleepy_table
            .into_iter()
            .map(|(id, x)| {
                (
                    id,
                    x.into_iter()
                        .filter(|(_, v)| !v.is_empty())
                        .fold(vec![], |acc, (_, v)| [acc, v].concat())
                        .iter()
                        .fold(HashMap::new(), |mut acc, n| {
                            {
                                let counter = acc.entry(*n).or_insert(0);

                                *counter += 1;
                            }

                            acc
                        })
                        .into_iter()
                        .max_by(|(_, x), (_, y)| x.cmp(y)),
                )
            })
            .filter_map(|(id, max)| match max {
                Some(n) => Some((id, n)),
                None => None,
            })
            .max_by(|(_, x), (_, y)| x.1.cmp(&y.1))
            .unwrap();

        println!("{:?}", sleepiest_guard.0 * (sleepiest_guard.1).0);
    }
}
