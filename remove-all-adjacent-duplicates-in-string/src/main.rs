impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut v = vec![];
        for c in s.chars() {
            if let Some(last) = v.last() {
                if last == &c {
                    v.pop();
                    continue;
                }
            }
            v.push(c);
        }
        v.into_iter().collect()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::remove_duplicates("abbaca".to_string()));
}
