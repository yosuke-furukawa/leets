impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut result = "".to_string();
        let mut count = 0;
        for c in s.chars() {
            if c == '(' && count > 0 {
                result.push(c);
            }
            if c == ')' && count > 1 {
                result.push(c);
            }
            count += if c == '(' { 1 } else { -1 };
        }
        result
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::remove_outer_parentheses("(()())(())".to_string())
    );
}
