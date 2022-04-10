impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut stack = vec![];
        for op in ops {
            match op.as_str() {
                "C" => {
                    stack.pop();
                }
                "D" => {
                    stack.push(stack[stack.len() - 1] * 2);
                }
                "+" => {
                    stack.push(stack[stack.len() - 2] + stack[stack.len() - 1]);
                }
                _ => {
                    stack.push(op.parse::<i32>().unwrap());
                }
            }
        }
        stack.iter().sum()
    }
}

struct Solution;

fn to_string_vec(v: Vec<&str>) -> Vec<String> {
    v.into_iter().map(|x| x.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::cal_points(to_string_vec(vec!["5", "2", "C", "D", "+"]))
    );
}
