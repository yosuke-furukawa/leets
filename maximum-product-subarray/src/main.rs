impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut max = nums[0];
        let mut min = nums[0];
        let mut result = max;

        for n in nums.into_iter().skip(1) {
            let temp_max = n.max((max * n).max(min * n));
            min = n.min((max * n).min(min * n));
            max = temp_max;
            result = result.max(max);
        }

        result
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_product(vec![2, 3, -2, 4]));
}
