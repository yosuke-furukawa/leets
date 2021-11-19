impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        hamming_weight(x ^ y)
    }
}

fn hamming_weight(bits: i32) -> i32 {
    let mut bits = bits;
    bits = (bits & 0x55555555) + (bits >> 1 & 0x55555555);
    bits = (bits & 0x33333333) + (bits >> 2 & 0x33333333);
    bits = (bits & 0x0f0f0f0f) + (bits >> 4 & 0x0f0f0f0f);
    bits = (bits & 0x00ff00ff) + (bits >> 8 & 0x00ff00ff);
    (bits & 0x0000ffff) + (bits >> 16 & 0x0000ffff)
}

struct Solution;

fn main() {
    println!("{}", Solution::hamming_distance(1, 4));
    println!("{}", Solution::hamming_distance(3, 1));
}
