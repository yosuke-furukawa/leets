const MAX: u64 = 1_000_000_007;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut dp: Vec<u64> = vec![0; s.len() + 1];
        let str: Vec<char> = s.chars().collect();
        dp[0] = 1;
        if str[0] == '*' {
            dp[1] = 9;
        } else if str[0] != '0' {
            dp[1] = 1;
        }

        for i in 1..str.len() {
            dp[i + 1] = match (str[i - 1], str[i]) {
                ('1', '*') => (9 * dp[i] % MAX + 9 * dp[i - 1] % MAX) % MAX,
                ('2', '*') => (9 * dp[i] % MAX + 6 * dp[i - 1] % MAX) % MAX,
                ('*', '*') => (9 * dp[i] % MAX + 15 * dp[i - 1] % MAX) % MAX,
                (_, '*') => 9 * dp[i] % MAX,
                ('1', '0') => dp[i - 1],
                ('2', '0') => dp[i - 1],
                ('*', '0') => 2 * dp[i - 1] % MAX,
                (_, '0') => 0,
                ('1', _) => (dp[i] + dp[i - 1]) % MAX,
                ('2', n) if n <= '6' => (dp[i] + dp[i - 1]) % MAX,
                ('*', n) if n <= '6' => (dp[i] + 2 * dp[i - 1] % MAX) % MAX,
                ('*', _) => (dp[i] + dp[i - 1]) % MAX,
                _ => dp[i],
            };
        }

        *dp.last().unwrap() as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::num_decodings("*".to_string()));
}
