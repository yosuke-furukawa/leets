impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut answer = vec![0; temperatures.len()];
        for (i, t) in temperatures.iter().enumerate().rev() {
            while !stack.is_empty() && t >= &temperatures[*stack.last().unwrap() as usize] {
                stack.pop();
            }

            answer[i] = if stack.is_empty() {
                0
            } else {
                stack.last().unwrap() - i as i32
            };
            stack.push(i as i32);
        }
        answer
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
    );
}
