impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(s.len());
        for _ in 0..s.len() {
            result.push(0);
        }
        let mut prev = std::i32::MIN / 2;

        for (i, ch) in s.chars().enumerate() {
            if ch == c {
                prev = i as i32;
            }
            result[i] = i as i32 - prev;
        }

        prev = std::i32::MAX / 2;
        for (j, ch) in s.chars().rev().enumerate() {
            let i = s.len() - j - 1;
            if ch == c {
                prev = i as i32;
            }
            result[i] = result[i].min(prev - i as i32);
        }
        result
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::shortest_to_char("loveleetcode".to_string(), 'e')
    );
}
