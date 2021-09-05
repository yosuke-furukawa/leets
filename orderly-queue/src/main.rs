impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        if k == 1 {
            let mut ans = s.clone();
            for i in 0..s.len() {
                let t = s.get(i..).unwrap().to_string() + s.get(0..i).unwrap();
                if t.cmp(&ans) == std::cmp::Ordering::Less {
                    ans = t;
                }
            }
            ans
        } else {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            chars
                .into_iter()
                .fold(String::new(), |acc, cur| acc + &cur.to_string())
        }
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::orderly_queue("baaca".to_string(), 3));
    println!("{}", Solution::orderly_queue("adguvpsubc".to_string(), 2));
}
