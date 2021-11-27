impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![1];
        for i in 1..nums.len() {
            answer.push(nums[i - 1] * answer[i - 1]);
        }

        let mut temp = 1;
        for i in (0..answer.len()).rev() {
            answer[i] *= temp;
            temp *= nums[i]
        }
        answer
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::product_except_self(vec![1, 2, 3, 4]));
    println!("{:?}", Solution::product_except_self(vec![-1, 1, 0, -3, 3]));
}
