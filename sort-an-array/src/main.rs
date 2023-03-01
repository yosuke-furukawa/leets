impl Solution {
    fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
        let mut result = Vec::with_capacity(left.len() + right.len());
        let mut l = 0;
        let mut r = 0;
        while l < left.len() || r < right.len() {
            if l == left.len() {
                result.push(right[r]);
                r += 1;
                continue;
            }
            if r == right.len() {
                result.push(left[l]);
                l += 1;
                continue;
            }
            if left[l] < right[r] {
                result.push(left[l]);
                l += 1;
            } else {
                result.push(right[r]);
                r += 1;
            }
        }
        result
    }
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let pivot = nums.len() / 2;
        if pivot == 0 {
            return nums;
        }
        let left = Self::sort_array(nums[..pivot].to_vec());
        let right = Self::sort_array(nums[pivot..].to_vec());
        Self::merge(&left, &right)
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::sort_array(vec![5, 2, 3, 1]));
}
