impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut players: Vec<(i32, i32)> = vec![];
        for i in 0..scores.len() {
            players.push((ages[i], scores[i]));
        }
        players.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        let mut dp = vec![0; players.len()];
        let mut res = 0;
        for i in 0..players.len() {
            dp[i] = players[i].1;
            for j in 0..i {
                if players[j].1 <= players[i].1 {
                    dp[i] = dp[i].max(dp[j] + players[i].1);
                }
            }
            res = res.max(dp[i]);
        }
        res
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::best_team_score(vec![1, 3, 5, 10, 15], vec![1, 2, 3, 4, 5])
    );
    println!(
        "{}",
        Solution::best_team_score(vec![4, 5, 6, 5], vec![2, 1, 2, 1])
    );
    println!(
        "{}",
        Solution::best_team_score(vec![1, 2, 3, 5], vec![8, 9, 10, 1])
    );
}
