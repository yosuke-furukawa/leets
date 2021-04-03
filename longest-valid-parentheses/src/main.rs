impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stock = vec![];
        let mut max = 0;
        let chars: Vec<char> = s.chars().collect();

        for (i, c) in chars.iter().enumerate() {
            if let Some(top) = stock.last() {
                if chars[*top] == '(' && *c == ')' {
                    stock.pop();
                    if let Some(new_top) = stock.last() {
                        max = max.max(i - *new_top);
                    } else {
                        max = max.max(i + 1);
                    }
                } else {
                    stock.push(i);
                }
            } else {
                stock.push(i);
            }
        }
        max as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::longest_valid_parentheses("(()".to_string()));
    println!(
        "{}",
        Solution::longest_valid_parentheses(")()())".to_string())
    );
    println!("{}", Solution::longest_valid_parentheses("".to_string()));
    println!("{}", Solution::longest_valid_parentheses("()".to_string()));
}
