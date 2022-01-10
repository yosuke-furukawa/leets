impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        format!(
            "{:b}",
            u128::from_str_radix(&a, 2).unwrap() + u128::from_str_radix(&b, 2).unwrap()
        )
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::add_binary("11".to_string(), "1".to_string())
    );
}
