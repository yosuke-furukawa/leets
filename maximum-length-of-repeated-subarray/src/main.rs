impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums2.len() + 1];
        let mut max = 0;
        for n1 in nums1.iter() {
            for (j, n2) in nums2.iter().enumerate().rev() {
                dp[j + 1] = if n1 == n2 { dp[j] + 1 } else { 0 };
            }
            max = max.max(*dp.iter().max().unwrap());
        }
        max
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7])
    );
    println!(
        "{}",
        Solution::find_length(vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0])
    );
    println!(
        "{}",
        Solution::find_length(vec![0, 0, 0, 0, 1], vec![1, 0, 0, 0, 0])
    );
}
