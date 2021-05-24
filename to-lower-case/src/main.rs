impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.to_lowercase()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::to_lower_case("Hello".to_string()));
    println!("{}", Solution::to_lower_case("here".to_string()));
    println!("{}", Solution::to_lower_case("LOVELY".to_string()));
}
