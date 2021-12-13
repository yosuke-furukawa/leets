impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut arr = vec![1; s.len()];
        let chars: Vec<char> = s.chars().collect();
        let mut max = 1;
        for i in 1..arr.len() {
            if chars[i] == chars[i - 1] {
                arr[i] = arr[i - 1] + 1;
                max = max.max(arr[i]);
            }
        }
        max
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_power("leetcode".to_string()));
}
