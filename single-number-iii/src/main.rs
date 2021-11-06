impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor = nums.iter().fold(0, |acc, x| acc ^ x);
        let right = xor & !(xor - 1);
        let mut ans = vec![0; 2];
        for n in nums.iter() {
            if n & right > 0 {
                ans[0] ^= n;
            } else {
                ans[1] ^= n;
            }
        }
        ans
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::single_number(vec![1, 2, 1, 3, 2, 5]));
}
