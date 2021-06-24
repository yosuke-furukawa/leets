impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let limit = 1_000_000_007;
        let mut dp = vec![vec![0; n as usize]; m as usize];
        dp[start_row as usize][start_column as usize] = 1;
        let mut count = 0;
        for _ in 1..=max_move {
            // println!("{:?}", dp);
            let mut temp = vec![vec![0; n as usize]; m as usize];
            for i in 0..m as usize {
                for j in 0..n as usize {
                    if i == m as usize - 1 {
                        count = (count + dp[i][j]) % limit;
                    }
                    if j == n as usize - 1 {
                        count = (count + dp[i][j]) % limit;
                    }
                    if i == 0 {
                        count = (count + dp[i][j]) % limit;
                    }
                    if j == 0 {
                        count = (count + dp[i][j]) % limit;
                    }
                    temp[i][j] = ((if i > 0 { dp[i - 1][j] } else { 0 }
                        + if i < m as usize - 1 { dp[i + 1][j] } else { 0 })
                        % limit
                        + (if j > 0 { dp[i][j - 1] } else { 0 }
                            + if j < n as usize - 1 { dp[i][j + 1] } else { 0 })
                            % limit)
                        % limit;
                }
            }
            dp = temp;
        }
        count
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find_paths(2, 2, 2, 0, 0));
}
