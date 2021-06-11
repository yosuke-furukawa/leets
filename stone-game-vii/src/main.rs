impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut dp = vec![vec![0; n]; n];
        let mut prefix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + stones[i];
        }

        for l in 2..=n {
            for s in 0..n - l + 1 {
                let e = s + l - 1;
                let score_remove_first = prefix_sum[e + 1] - prefix_sum[s + 1];
                let score_remove_last = prefix_sum[e] - prefix_sum[s];
                dp[s][e] =
                    (score_remove_first - dp[s + 1][e]).max(score_remove_last - dp[s][e - 1]);
            }
        }
        dp[0][n - 1]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::stone_game_vii(vec![5, 3, 1, 4, 2]));
    println!(
        "{}",
        Solution::stone_game_vii(vec![7, 90, 5, 1, 100, 10, 10, 2])
    );
}
