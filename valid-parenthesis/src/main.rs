impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '(' | '{' | '[' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                _ => continue,
            }
        }
        stack.len() == 0
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::is_valid("(){}[]".to_string()));
    println!("{}", Solution::is_valid("[({})]".to_string()));
}
