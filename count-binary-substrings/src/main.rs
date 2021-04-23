impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut count = 0;
        let mut prev = 0;
        let mut curr = 1;
        let chars: Vec<char> = s.chars().collect();
        for c in chars.windows(2) {
            if c[0] == c[1] {
                curr += 1;
            } else {
                count += curr.min(prev);
                prev = curr;
                curr = 1;
            }
        }
        count + curr.min(prev)
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::count_binary_substrings("00110011".to_string())
    );
}
