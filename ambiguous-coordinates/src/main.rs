use std::iter::FromIterator;

trait Nums {
    fn nums(self) -> Vec<String>;
}

impl Nums for &[char] {
    fn nums(self) -> Vec<String> {
        let n = self.len();
        let mut res = vec![];
        for i in 1..=n {
            let left = String::from_iter(self[..i].iter());
            let right = String::from_iter(self[i..].iter());
            if left.starts_with('0') && left != "0" {
                continue;
            }
            if right.ends_with('0') {
                continue;
            }
            res.push(format!(
                "{}{}{}",
                left,
                if i == n { "" } else { "." },
                right
            ));
        }
        res
    }
}

impl Solution {
    fn ambiguous_coordinates(s: String) -> Vec<String> {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut res = vec![];
        for i in 2..n - 1 {
            let left = &s[1..i];
            let right = &s[i..n - 1];
            for l in left.nums() {
                for r in right.nums() {
                    res.push(format!("({}, {})", l, r));
                }
            }
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::ambiguous_coordinates("(123)".to_string()));
    println!(
        "{:?}",
        Solution::ambiguous_coordinates("(0123)".to_string())
    );
}
