impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let mut res = vec![];
        for num in nums.windows(k as usize) {
            let mut num = num.to_vec();
            num.sort_unstable();
            if k % 2 == 0 {
                res.push((num[k as usize / 2 - 1] as f64 + num[k as usize / 2] as f64) / 2.0);
            } else {
                res.push(num[k as usize / 2] as f64);
            }
        }
        res
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::median_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
    );
}
