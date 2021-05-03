impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];
        for (i, n) in nums.into_iter().enumerate() {
            match i {
                0 => {
                    ans[i] = n;
                }
                _ => {
                    ans[i] = ans[i - 1] + n;
                }
            }
        }
        ans
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::running_sum(vec![1, 2, 3, 4]));
}
