impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let mut dp = vec![vec![vec![0.0; n as usize]; n as usize]; k as usize + 1];
        dp[0][row as usize][column as usize] = 1.0;
        for i in 1..=k as usize {
            for r in 0..n as usize {
                for c in 0..n as usize {
                    for (dr, dc) in &[(2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2)] {
                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;
                        if nr >= 0 && nr < n && nc >= 0 && nc < n {
                            dp[i][nr as usize][nc as usize] += dp[i - 1][r][c] / 8.0;
                        }
                    }
                }
            }
        }
        dp[k as usize].iter().flatten().sum()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::knight_probability(3, 2, 0, 0));
}
