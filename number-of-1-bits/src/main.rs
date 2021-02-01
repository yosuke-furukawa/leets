impl Solution {
    pub fn hammingWeight(mut bits: u32) -> i32 {
        bits = (bits & 0x55555555) + (bits >> 1 & 0x55555555);
        bits = (bits & 0x33333333) + (bits >> 2 & 0x33333333);
        bits = (bits & 0x0f0f0f0f) + (bits >> 4 & 0x0f0f0f0f);
        bits = (bits & 0x00ff00ff) + (bits >> 8 & 0x00ff00ff);
        ((bits & 0x0000ffff) + (bits >>16 & 0x0000ffff)) as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::hammingWeight(1));
}
