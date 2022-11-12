impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }
        let mut res = vec![];
        for &n in nums.iter() {
            if n != 0 {
                res.push(n);
            }
        }
        for _ in res.len()..nums.len() {
            res.push(0);
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::apply_operations(vec![1, 2, 2, 1, 1, 0]));
    println!("{:?}", Solution::apply_operations(vec![0, 1]));
}
