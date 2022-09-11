impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let x = (n ^ n >> 1) + 1;
        x & (x - 1) == 0
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::has_alternating_bits(5));
    println!("{}", Solution::has_alternating_bits(7));
    println!("{}", Solution::has_alternating_bits(11));
}
