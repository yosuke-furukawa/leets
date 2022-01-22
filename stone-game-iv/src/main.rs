impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut dp = vec![false; n as usize + 1];
        for i in 0..=n as usize {
            if dp[i] {
                continue;
            }
            let mut j = 1;
            while i + j * j <= n as usize {
                dp[i + j * j] = true;
                j += 1;
            }
        }
        dp[n as usize]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::winner_square_game(1));
    println!("{}", Solution::winner_square_game(2));
    println!("{}", Solution::winner_square_game(4));
}
