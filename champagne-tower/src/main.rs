impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut dp = vec![0.0; 101];
        dp[0] = poured as f64;
        for i in 1..=query_row as usize {
            for j in (0..=i).rev() {
                dp[j] = ((dp[j] - 1.0) / 2.0).max(0.0);
                dp[j + 1] += dp[j];
            }
        }
        dp[query_glass as usize].min(1.0)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::champagne_tower(1, 1, 1));
    println!("{}", Solution::champagne_tower(2, 1, 1));
    println!("{}", Solution::champagne_tower(100000009, 33, 17));
}
