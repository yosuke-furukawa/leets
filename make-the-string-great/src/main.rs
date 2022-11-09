impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if let Some(&l) = stack.last() {
                if l != c && l.to_ascii_lowercase() == c.to_ascii_lowercase() {
                    stack.pop();
                    continue;
                }
            }
            stack.push(c);
        }
        stack.into_iter().collect()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::make_good("leEeetcode".to_string()));
    println!("{}", Solution::make_good("abBAcC".to_string()));
    println!("{}", Solution::make_good("s".to_string()));
    println!("{}", Solution::make_good("Pp".to_string()));
}
