impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let mut tables = vec![vec![false; n]; n];
        for i in 0..n {
            tables[i][i] = true;
        }
        let mut start = 0;
        let mut max = 1;
        for i in 0..n - 1 {
            if s.get(i..i + 1) == s.get(i + 1..i + 2) {
                tables[i][i + 1] = true;
                start = i;
                max = 2;
            }
        }
        for k in 3..=n {
            for i in 0..n - k + 1 {
                let j = i + k - 1;
                if tables[i + 1][j - 1] && s.get(i..i + 1) == s.get(j..j + 1) {
                    tables[i][j] = true;

                    if k > max {
                        start = i;
                        max = k;
                    }
                }
            }
        }
        let end = start + max;
        s.get(start..end).into_iter().collect()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::longest_palindrome("babad".to_string()));
    println!("{}", Solution::longest_palindrome("cbbd".to_string()));
    println!("{}", Solution::longest_palindrome("a".to_string()));
}
