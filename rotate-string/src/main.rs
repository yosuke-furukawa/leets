impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let g = goal.chars().collect::<Vec<char>>();
        if s.len() != goal.len() {
            return false;
        }
        let num = s.len();
        let mut i = 0;
        let mut j = 0;
        while j < g.len() {
            if s[i] != g[j] {
                j += 1;
                continue;
            }
            let mut k = j;
            while i < num && s[i] == g[k] {
                i += 1;
                k = (k + 1) % num;
            }
            if i == num {
                return true;
            }
            i = 0;
            j += 1;
        }
        false
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::rotate_string("abcde".to_string(), "cdeab".to_string())
    );
    println!(
        "{}",
        Solution::rotate_string("abcde".to_string(), "abced".to_string())
    );
    println!(
        "{}",
        Solution::rotate_string("bbbacddceeb".to_string(), "ceebbbbacdd".to_string())
    );
}
