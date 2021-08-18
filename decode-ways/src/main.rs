impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut dp = vec![0; s.len()];

        if !s.starts_with('0') {
            dp[0] = 1;
        }

        for (i, c) in s.chars().enumerate().skip(1) {
            let x = c as u32 - '0' as u32;
            let y = s.get(i - 1..i + 1).unwrap().parse::<u32>().unwrap();
            if (1..=9).contains(&x) {
                dp[i] += dp[i - 1];
            }
            if (10..=26).contains(&y) {
                if i >= 2 {
                    dp[i] += dp[i - 2];
                } else {
                    dp[i] += 1;
                }
            }
        }

        dp[s.len() - 1]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::num_decodings("12".to_string()));
    println!("{}", Solution::num_decodings("226".to_string()));
    println!("{}", Solution::num_decodings("0".to_string()));
    println!("{}", Solution::num_decodings("06".to_string()));
}
