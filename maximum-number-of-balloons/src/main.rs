use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut count_map = HashMap::new();
        for c in text.chars() {
            *count_map.entry(c).or_insert(0) += 1;
        }
        let mut count = 0;
        let mut keep = true;
        while keep {
            for c in "balloon".chars() {
                let entry = count_map.entry(c).or_insert(0);
                if *entry == 0 {
                    keep = false;
                    break;
                }
                *entry -= 1;
            }
            if keep {
                count += 1;
            }
        }
        count
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::max_number_of_balloons("nlaebolko".to_string())
    );
    println!(
        "{}",
        Solution::max_number_of_balloons("loonbalxballpoon".to_string())
    );
    println!(
        "{}",
        Solution::max_number_of_balloons("leetcode".to_string())
    );
}
