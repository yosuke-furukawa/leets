use std::collections::VecDeque;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut tokens = VecDeque::from(tokens);
        let mut nums = vec![];
        while !tokens.is_empty() {
            let token = tokens.pop_front().unwrap();
            let result = match token.as_str() {
                "+" => nums.pop().unwrap() + nums.pop().unwrap(),
                "-" => {
                    let a = nums.pop().unwrap();
                    let b = nums.pop().unwrap();
                    b - a
                }
                "*" => nums.pop().unwrap() * nums.pop().unwrap(),
                "/" => {
                    let a = nums.pop().unwrap();
                    let b = nums.pop().unwrap();
                    b / a
                }
                _ => token.parse::<i32>().unwrap(),
            };
            nums.push(result);
        }
        nums[0]
    }
}

struct Solution;

fn stringify(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::eval_rpn(stringify(vec!["2", "1", "+", "3", "*"]))
    );
    println!(
        "{}",
        Solution::eval_rpn(stringify(vec!["4", "13", "5", "/", "+"]))
    );
    println!(
        "{}",
        Solution::eval_rpn(stringify(vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
        ]))
    );
}
