impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut left = nums.len();
        let mut right = 0;

        for i in 0..nums.len() {
            while !stack.is_empty() && nums[stack[stack.len() - 1]] > nums[i] {
                left = left.min(stack.pop().unwrap());
            }
            stack.push(i);
        }

        stack.clear();

        for i in (0..nums.len()).rev() {
            while !stack.is_empty() && nums[stack[stack.len() - 1]] < nums[i] {
                right = right.max(stack.pop().unwrap());
            }
            stack.push(i);
        }
        (match (left, right) {
            (l, r) if r > l => r - l + 1,
            _ => 0,
        }) as i32
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15])
    );
}
