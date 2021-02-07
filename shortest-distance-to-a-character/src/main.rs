impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(s.len());
        for _ in 0..s.len() {
            result.push(1000000);
        }
        for (i, ch) in s.chars().enumerate() {
            if ch == c {
                result[i] = 0;
                let mut left = (i-1) as i32;
                while left >= 0 && result[left as usize] != 0 {
                    result[left as usize] = result[left as usize].min(i as i32 - left);
                    left -= 1;
                }
                let mut right = (i+1) as i32;
                while right < s.len() as i32 && result[right as usize] != 0 {
                    result[right as usize] = result[right as usize].min(right - i as i32);
                    right += 1;
                }
            }
        }
        result
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::shortest_to_char("loveleetcode".to_string(), 'e'));
}
