impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut max_time = release_times[0];
        let chars: Vec<char> = keys_pressed.chars().collect();
        let mut max_char = chars[0];
        for (i, c) in chars.iter().enumerate().skip(1) {
            let d = release_times[i] - release_times[i - 1];
            if max_time < d || max_time == d && max_char < *c {
                max_char = *c;
                max_time = d;
            }
        }
        max_char
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::slowest_key(vec![9, 29, 49, 50], "cbcd".to_string())
    );
    println!(
        "{}",
        Solution::slowest_key(vec![12, 23, 36, 46, 62], "spuda".to_string())
    );
}
