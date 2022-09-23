impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = 0;
        while i < bits.len() - 1 {
            i += if bits[i] == 0 { 1 } else { 2 };
        }
        i == bits.len() - 1
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::is_one_bit_character(vec![1, 0, 0]));
}
