impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut count = vec![0; 26];
        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }
        for (i, c) in s.chars().enumerate() {
            if count[(c as u8 - b'a') as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::first_uniq_char("leetcode".to_string()));
}
