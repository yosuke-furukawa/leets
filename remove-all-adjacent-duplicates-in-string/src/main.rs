impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut v = vec![];
        for c in s.chars() {
            if v.is_empty() {
                v.push(c);
                continue;
            }
            let last = v.last().unwrap();
            if last == &c {
                v.pop();
                continue;
            }
            v.push(c);
        }
        v.into_iter()
            .fold(String::new(), |acc, cur| acc + &cur.to_string())
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::remove_duplicates("abbaca".to_string()));
}
