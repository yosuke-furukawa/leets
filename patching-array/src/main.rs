impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let n = n as i64;
        let nums: Vec<i64> = nums.into_iter().map(|n| n as i64).collect();
        let mut result = 0;
        let mut miss = 1 as i64;
        let mut i = 0;
        while miss <= n {
            if i < nums.len() && nums[i] <= miss {
                miss += nums[i];
                i += 1;
            } else {
                miss += miss;
                result += 1;
            }
        }
        result
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_patches(vec![1, 3], 6));
}
