impl Solution {
    fn is_additive_number(num: String) -> bool {
        let n = num.len();
        let mut cur: Vec<u64> = vec![];
        Self::dfs(0, &mut cur, &num[0..n], n)
    }

    fn dfs(start: usize, cur: &mut Vec<u64>, s: &str, n: usize) -> bool {
        if start == n {
            if cur.len() >= 3 {
                return true;
            }
        } else {
            for i in start + 1..=n {
                if &s[start..=start] == "0" && i - start > 1 {
                    return false;
                }
                if let Ok(x) = s[start..i].parse::<u64>() {
                    let k = cur.len();
                    if k < 2 || cur[k - 1] + cur[k - 2] == x {
                        cur.push(x);
                        if Self::dfs(i, cur, s, n) {
                            return true;
                        }
                        cur.pop();
                    }
                }
            }
        }
        false
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::is_additive_number("112358".to_string()));
    println!("{}", Solution::is_additive_number("199100199".to_string()));
}
