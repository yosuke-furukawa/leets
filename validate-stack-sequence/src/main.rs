impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack: Vec<i32> = Vec::with_capacity(pushed.len());
        let mut popped_index = 0;
        for n in pushed {
            stack.push(n);
            while !stack.is_empty() && stack[stack.len() - 1] == popped[popped_index] {
                stack.pop();
                popped_index += 1;
            }
        }
        stack.is_empty()
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1])
    );
    println!(
        "{}",
        Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2])
    );
}
