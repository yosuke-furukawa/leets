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
                _ => return false,
            }
        }
        stack.is_empty()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::is_valid("(){}[]".to_string()));
    println!("{}", Solution::is_valid("[({})]".to_string()));
}
