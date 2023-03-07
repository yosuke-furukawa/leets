impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let total_trips = total_trips as i128;
        let time: Vec<i128> = time.into_iter().map(|x| x as i128).collect();
        let mut left: i128 = 0;
        let mut right: i128 = 1_000_000_000_000_000_000;
        while left < right {
            let mid = (left + right) / 2;
            let mut trips = 0;
            for t in &time {
                trips += mid / t;
            }
            if trips >= total_trips {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as i64
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::minimum_time(vec![1, 2, 3], 5));
    println!("{}", Solution::minimum_time(vec![2], 1));
    println!("{}", Solution::minimum_time(vec![5, 10, 10], 9));
}
