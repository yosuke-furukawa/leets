fn to_int(time_points: Vec<String>) -> Vec<i32> {
    time_points
        .iter()
        .map(|time_point| {
            let time_point = time_point.split(':').collect::<Vec<&str>>();
            let hour = time_point[0].parse::<i32>().unwrap();
            let minute = time_point[1].parse::<i32>().unwrap();
            hour * 60 + minute
        })
        .collect::<Vec<i32>>()
}

fn diff(time1: i32, time2: i32) -> i32 {
    if time2 >= time1 {
        time2 - time1
    } else {
        24 * 60 - time1 + time2
    }
}

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut minutes = to_int(time_points);
        minutes.sort_unstable();
        let mut min_diff = std::i32::MAX;
        for i in 0..minutes.len() as i32 {
            let c = i as usize;
            let p = if c == 0 { minutes.len() - 1 } else { c - 1 };
            let n = if c == minutes.len() - 1 { 0 } else { c + 1 };
            min_diff = min_diff
                .min(diff(minutes[p], minutes[c]))
                .min(diff(minutes[c], minutes[n]));
        }
        min_diff
    }
}

struct Solution;

fn stringify(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::find_min_difference(stringify(vec!["23:59", "00:00"]))
    );
    println!(
        "{}",
        Solution::find_min_difference(stringify(vec!["01:01", "02:01", "03:00"]))
    );
}
