impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let limit = 1_000_000_007;
        let mut dp = vec![vec![0; n as usize]; m as usize];
        dp[start_row as usize][start_column as usize] = 1;
        let mut count = 0;
        for _ in 0..=max_move {
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
                    for d in [0, 1, 0, !0, 0].windows(2) {
                        let row = (i as i32).wrapping_add(d[0]);
                        let col = (j as i32).wrapping_add(d[1]);
                        if (0..m).contains(&row) && (0..n).contains(&col) {
                            temp[row as usize][col as usize] =
                                (temp[row as usize][col as usize] + dp[i][j]) % limit;
                        }
                    }
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
