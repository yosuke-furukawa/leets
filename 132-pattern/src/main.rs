impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 3 {
            return false;
        }
        let mut stack = vec![];
        let mut min = vec![nums[0]];
        for i in 1..n {
            min.push(min[i - 1].min(nums[i]));
        }
        for i in (0..n).rev() {
            if nums[i] > min[i] {
                while !stack.is_empty() && stack.last().unwrap() <= &min[i] {
                    stack.pop();
                }
                if !stack.is_empty() && stack.last().unwrap() < &nums[i] {
                    return true;
                }
                stack.push(nums[i]);
            }
        }
        false
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find132pattern(vec![1, 2, 3, 4]));
    println!("{}", Solution::find132pattern(vec![3, 1, 4, 2]));
    println!("{}", Solution::find132pattern(vec![-1, 3, 2, 0]));
}
