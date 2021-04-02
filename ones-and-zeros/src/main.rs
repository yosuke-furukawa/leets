impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; (n + 1) as usize]; (m + 1) as usize];
        for str in strs {
            let mut zeros = 0;
            let mut ones = 0;
            for c in str.chars() {
                if c == '0' {
                    zeros += 1;
                } else {
                    ones += 1;
                }
            }
            for z in (zeros..=m as usize).rev() {
                for o in (ones..=n as usize).rev() {
                    dp[z][o] = (1 + dp[z - zeros][o - ones]).max(dp[z][o]);
                }
            }
        }
        dp[m as usize][n as usize]
    }
}

struct Solution;

fn to_string(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::find_max_form(to_string(vec!["10", "0001", "111001", "1", "0"]), 5, 3)
    );
    println!(
        "{}",
        Solution::find_max_form(to_string(vec!["10", "0", "1"]), 1, 1)
    );
}
