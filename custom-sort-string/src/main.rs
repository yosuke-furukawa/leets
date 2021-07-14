use std::collections::HashMap;

impl Solution {
    pub fn custom_sort_string(order: String, str: String) -> String {
        let mut order_map = HashMap::new();
        for (i, o) in order.chars().enumerate() {
            order_map.insert(o, i);
        }

        let mut chars: Vec<char> = str.chars().collect();
        chars.sort_unstable_by(|a, b| {
            order_map
                .get(a)
                .unwrap_or(&1_000_000)
                .cmp(&order_map.get(b).unwrap_or(&1_000_000))
        });
        chars.iter().collect()
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::custom_sort_string("cba".to_string(), "abcd".to_string())
    );
}
