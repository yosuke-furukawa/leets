impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let st: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut count = 0;

        let mut dp: Vec<Vec<bool>> = vec![vec![false; n]; n];

        for i in 0..n {
            dp[i][i] = true;
        }

        for l in 2..=n {
            for i in 0..=n-l {
                let j = i + l - 1;

                if l == 2 {
                    if st[i] == st[j] {
                        dp[i][j] = true;
                    }
                }

                if st[i] == st[j] && dp[i+1][j-1] {
                    dp[i][j] = true;
                }
            }
        }

        for i in 0..dp.len() {
            for j in 0..dp[i].len() {
                if dp[i][j] {
                    count += 1;
                }
            }
        }

        count
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::count_substrings("abc".to_string()));
}
