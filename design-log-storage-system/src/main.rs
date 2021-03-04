use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::Bound::Included;

#[derive(Default)]
struct LogSystem {
    logs: BTreeMap<Time, i32>,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Time {
    year: usize,
    month: usize,
    date: usize,
    hour: usize,
    minute: usize,
    second: usize,
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        let year_cmp = self.year.cmp(&other.year);
        let month_cmp = self.month.cmp(&other.month);
        let date_cmp = self.date.cmp(&other.date);
        let hour_cmp = self.hour.cmp(&other.hour);
        let minute_cmp = self.minute.cmp(&other.minute);
        let second_cmp = self.second.cmp(&other.second);

        match (
            year_cmp, month_cmp, date_cmp, hour_cmp, minute_cmp, second_cmp,
        ) {
            (
                Ordering::Equal,
                Ordering::Equal,
                Ordering::Equal,
                Ordering::Equal,
                Ordering::Equal,
                _,
            ) => second_cmp,
            (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Equal, _, _) => {
                minute_cmp
            }
            (Ordering::Equal, Ordering::Equal, Ordering::Equal, _, _, _) => hour_cmp,
            (Ordering::Equal, Ordering::Equal, _, _, _, _) => date_cmp,
            (Ordering::Equal, _, _, _, _, _) => month_cmp,
            _ => year_cmp,
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LogSystem {
    fn new() -> Self {
        Self::default()
    }

    fn put(&mut self, id: i32, timestamp: String) {
        let time = LogSystem::parse_time(timestamp);
        self.logs.insert(time, id);
    }

    fn parse_time(time: String) -> Time {
        let times = time
            .split(':')
            .collect::<Vec<&str>>()
            .iter()
            .map(|str| str.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Time {
            year: times[0],
            month: times[1],
            date: times[2],
            hour: times[3],
            minute: times[4],
            second: times[5],
        }
    }

    fn retrieve(&self, start: String, end: String, granularity: String) -> Vec<i32> {
        let mut start = LogSystem::parse_time(start);
        let mut end = LogSystem::parse_time(end);
        match granularity.as_str() {
            "Year" => {
                start.month = 0;
                start.date = 0;
                start.hour = 0;
                start.minute = 0;
                start.second = 0;
                end.month = 12;
                end.date = 31;
                end.hour = 23;
                end.minute = 59;
                end.second = 59;
            }
            "Month" => {
                start.date = 0;
                start.hour = 0;
                start.minute = 0;
                start.second = 0;
                end.date = 31;
                end.hour = 23;
                end.minute = 59;
                end.second = 59;
            }
            "Day" => {
                start.hour = 0;
                start.minute = 0;
                start.second = 0;
                end.hour = 23;
                end.minute = 59;
                end.second = 59;
            }
            "Hour" => {
                start.minute = 0;
                start.second = 0;
                end.minute = 59;
                end.second = 59;
            }
            "Minute" => {
                start.second = 0;
                end.second = 59;
            }
            _ => {}
        }
        self.logs.range((Included(start), Included(end))).map(|(_, v)| *v).collect()
    }
}

/**
 * Your LogSystem object will be instantiated and called as such:
 * let obj = LogSystem::new();
 * obj.put(id, timestamp);
 * let ret_2: Vec<i32> = obj.retrieve(start, end, granularity);
 */

fn main() {
    let mut obj = LogSystem::new();
    obj.put(1, "2017:01:01:23:59:59".to_string());
    obj.put(2, "2017:01:01:22:59:59".to_string());
    obj.put(3, "2016:01:01:00:00:00".to_string());
    println!(
        "{:?}",
        obj.retrieve(
            "2016:01:01:01:01:01".to_string(),
            "2017:01:01:23:00:00".to_string(),
            "Year".to_string()
        )
    );
    println!(
        "{:?}",
        obj.retrieve(
            "2016:01:01:01:01:01".to_string(),
            "2017:01:01:23:00:00".to_string(),
            "Hour".to_string()
        )
    );
}
