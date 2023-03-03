impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if let Some(v) = haystack.find(&needle) {
            v as i32
        } else {
            -1
        }
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::str_str("sadbutsad".to_string(), "sad".to_string())
    );
    println!(
        "{}",
        Solution::str_str("hello".to_string(), "ll".to_string())
    );
}
