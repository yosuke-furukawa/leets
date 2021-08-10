    impl Solution {
        pub fn min_flips_mono_incr(s: String) -> i32 {
            let n = s.len();
            let mut dp = vec![0; n + 1];
            for i in 0..n {
                if let Some(v) = s.get(i..i + 1) {
                    dp[i + 1] = dp[i]
                        + match v {
                            "1" => 1,
                            _ => 0,
                        };
                }
            }
            let mut ans = std::i32::MAX;
            for j in 0..=n {
                ans = ans.min(dp[j] + n as i32 - j as i32 - (dp[n] - dp[j]));
            }
            ans
        }
    }

struct Solution;

fn main() {
    println!("{}", Solution::min_flips_mono_incr("00110".to_string()));
    println!("{}", Solution::min_flips_mono_incr("010110".to_string()));
    println!("{}", Solution::min_flips_mono_incr("00011000".to_string()));
    println!(
        "{}",
        Solution::min_flips_mono_incr("0101100011".to_string())
    );
}
