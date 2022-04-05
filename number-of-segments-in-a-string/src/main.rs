impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split_ascii_whitespace().count() as i32
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::count_segments("Hello, my name is John".to_string())
    );
}
