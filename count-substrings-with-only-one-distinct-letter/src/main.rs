impl Solution {
    pub fn count_letters(s: String) -> i32 {
        let mut buf = "".to_string();
        let mut result = vec![];
        let s = s + "\n";
        for c in s.chars() {
            if buf.len() == 0 || buf.contains(c) {
                buf += &c.to_string();
            } else {
                result.push(buf);
                buf = c.to_string();
            }
        }
        let mut ans = 0;
        for subs in result {
            let n = subs.len();
            ans += (n * (n + 1)) / 2;
        }
        ans as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::count_letters("aaaba".to_string()));
    println!("{}", Solution::count_letters("aaaaaaaaaa".to_string()));
}
