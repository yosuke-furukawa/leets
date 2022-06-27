impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                let x = nums[i];
                let y = nums[j];
                let z = x ^ y;
                sum += z.count_ones();
            }
        }
        sum as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::total_hamming_distance(vec![4, 14, 2]));
}
