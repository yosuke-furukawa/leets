impl Solution {
    fn gcd(a: i128, b: i128) -> i128 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
    fn lcm(a: i128, b: i128) -> i128 {
        a * b / Self::gcd(a, b)
    }
    pub fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            let mut lcm = nums[i] as i128;
            for j in i..nums.len() {
                lcm = Self::lcm(lcm as i128, nums[j] as i128);
                if lcm == k as i128 {
                    count += 1;
                }
            }
        }
        count
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::subarray_lcm(vec![3, 6, 2, 7, 1], 6));
    println!("{}", Solution::subarray_lcm(vec![773, 613, 11, 8, 103], 40));
    println!(
        "{}",
        Solution::subarray_lcm(vec![2, 729, 19, 841, 961, 151, 283, 443, 449, 509], 242)
    );
}
