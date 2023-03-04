impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut res = 0;
        for chunk in nums.split(|n| *n < min_k || *n > max_k) {
            let mut slow = 0;
            let (mut min_i_opt, mut max_i_opt) = (None, None);
            for fast in 0..chunk.len() {
                if chunk[fast] == min_k {
                    min_i_opt = Some(fast);
                }
                if chunk[fast] == max_k {
                    max_i_opt = Some(fast);
                }
                if let (Some(min_i), Some(max_i)) = (min_i_opt, max_i_opt) {
                    let (lo, hi) = (min_i.min(max_i), min_i.max(max_i));
                    res += (lo - slow + 1) as u64 * (chunk.len() - hi) as u64;
                    if fast == min_i {
                        slow = max_i + 1;
                        max_i_opt = None;
                    } else {
                        slow = min_i + 1;
                        min_i_opt = None;
                    }
                }
            }
        }
        res as i64
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5)
    );
}
