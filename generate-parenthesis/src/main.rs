impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut answer = vec![];
        if n == 0 {
            answer.push(String::new());
            return answer;
        }
        for c in 0..n {
            for left in Solution::generate_parenthesis(c) {
                for right in Solution::generate_parenthesis(n - 1 - c) {
                    answer.push(format!("({}){}", left, right));
                }
            }
        }
        answer
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::generate_parenthesis(3));
}
