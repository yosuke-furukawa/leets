const CHARS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let mut chars = Vec::with_capacity(n as usize);
        let mut max = k - n;
        for _ in 0..n {
            chars.push(match max {
                m if m >= 25 => {
                    max -= 25;
                    'z'
                }
                m if m < 25 && m >= 1 => {
                    let c = max;
                    max -= max;
                    CHARS[c as usize]
                }
                _ => 'a',
            });
        }
        chars.into_iter().rev().collect()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::get_smallest_string(3, 27));
    println!("{}", Solution::get_smallest_string(5, 73));
}
