impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut result: u128 = 0;

        for i in 1..=n {
            let mut k: u128 = i as u128;
            let mut size: u128 = 0;
            while k > 0 {
                size += 1;
                k >>= 1;
            }
            result = ((result << size) | i as u128) % 1000000007;
        }
        result as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::concatenated_binary(1));
    println!("{}", Solution::concatenated_binary(3));
    println!("{}", Solution::concatenated_binary(12));
    println!("{}", Solution::concatenated_binary(5));
    println!("{}", Solution::concatenated_binary(100000));
}
