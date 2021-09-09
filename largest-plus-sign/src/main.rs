use std::collections::HashSet;

impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let mut zeroes = HashSet::new();
        for mine in &mines {
            zeroes.insert((mine[0] * n + mine[1]) as usize);
        }
        let mut dp = vec![vec![0; n as usize]; n as usize];
        let mut ans = 0;

        for r in 0..n as usize {
            let mut count = 0;
            for c in 0..n as usize {
                count = if zeroes.contains(&(r * n as usize + c)) {
                    0
                } else {
                    count + 1
                };
                dp[r][c] = count;
            }
            count = 0;
            for c in (0..n as usize).rev() {
                count = if zeroes.contains(&(r * n as usize + c)) {
                    0
                } else {
                    count + 1
                };
                dp[r][c] = count.min(dp[r][c]);
            }
        }

        for c in 0..n as usize {
            let mut count = 0;
            for r in 0..n as usize {
                count = if zeroes.contains(&(r * n as usize + c)) {
                    0
                } else {
                    count + 1
                };
                dp[r][c] = count.min(dp[r][c]);
            }
            count = 0;
            for r in (0..n as usize).rev() {
                count = if zeroes.contains(&(r * n as usize + c)) {
                    0
                } else {
                    count + 1
                };
                dp[r][c] = count.min(dp[r][c]);
                ans = ans.max(dp[r][c]);
            }
        }

        ans
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::order_of_largest_plus_sign(5, vec![vec![4, 2]])
    );
}
