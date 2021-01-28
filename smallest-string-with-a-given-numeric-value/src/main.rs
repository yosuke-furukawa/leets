impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let mut chars = vec![0u8; n as _];
        let mut max = k - n;
        for c in chars.iter_mut().rev() {
            let offset = match max {
                max if max >= 25 => 25,
                max if max < 25 && max > 0 => max,
                _ => 0,   
            } as u8;
            *c = b'a' + offset as u8;
            max -= offset as i32;
        }
        unsafe { String::from_utf8_unchecked(chars) }
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::get_smallest_string(3, 27));
    println!("{}", Solution::get_smallest_string(5, 73));
}
