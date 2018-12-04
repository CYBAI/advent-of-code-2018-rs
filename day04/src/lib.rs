extern crate aoc_utils;
extern crate regex;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::cmp::Ordering;
use std::fmt;

lazy_static! {
    pub static ref FIND_GUARD_REGEX: Regex =
        Regex::new(r"\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})\] (?:Guard \#(?P<guard_id>\d+) )?(?P<action>.*)",)
            .unwrap();
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action {
    Begin,
    Asleep,
    Awake,
}

impl Action {
    pub fn parse_str_to_action(action_str: &str) -> Result<Action, ()> {
        match action_str {
            "begins shift" => Ok(Action::Begin),
            "falls asleep" => Ok(Action::Asleep),
            "wakes up" => Ok(Action::Awake),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Time {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
}

impl Time {
    pub fn empty() -> Self {
        Time {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
        }
    }

    pub fn new(year: i32, month: i32, day: i32, hour: i32, minute: i32) -> Self {
        Time {
            year,
            month,
            day,
            hour,
            minute,
        }
    }

    fn is_leap_year(&self) -> bool {
        ((self.year % 4 == 0) && (self.year % 100 != 0)) || (self.year % 400 == 0)
    }

    pub fn get_hour(&self) -> i32 {
        self.hour
    }

    pub fn get_minute(&self) -> i32 {
        self.minute
    }

    pub fn set_to_next_day(&mut self) {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 if self.day == 31 => {
                if self.month == 12 {
                    self.year += 1;
                    self.month = 1;
                }

                self.day = 0;
            }
            4 | 6 | 9 | 11 if self.day == 30 => {
                self.month += 1;
                self.day = 0;
            }
            2 if self.day == 28 && !self.is_leap_year() => {
                self.month += 1;
                self.day = 0;
            }
            _ => {}
        }

        self.day += 1;
        self.hour = 0;
        self.minute = -1;
    }

    pub fn get_date_string(&self) -> String {
        format!("{}-{}-{}", self.year, self.month, self.day)
    }
}

impl fmt::Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Time {{ {}-{}-{} {}:{} }}",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Guard {
    time: Time,
    guard_id: Option<i32>,
    action: Action,
}

impl Guard {
    pub fn new(time: Time, guard_id: Option<i32>, action: Action) -> Self {
        Guard {
            time,
            guard_id,
            action,
        }
    }

    pub fn get_action(&self) -> Action {
        self.action.clone()
    }

    pub fn get_time(&self) -> Time {
        self.time.clone()
    }

    pub fn get_guard_id(&self) -> Option<i32> {
        self.guard_id
    }
}

impl Ord for Guard {
    fn cmp(&self, other: &Guard) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for Guard {
    fn partial_cmp(&self, other: &Guard) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
