impl Solution {
    pub fn bitwise_complement(num: i32) -> i32 {
        if num == 0 {
            return 1;
        }
        let mut bin = String::new();
        let mut num = num;
        while num > 0 {
            if num % 2 == 0 {
                bin = "1".to_string() + &bin;
            } else {
                bin = "0".to_string() + &bin;
            }
            num /= 2;
        }
        i32::from_str_radix(&bin, 2).unwrap()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::bitwise_complement(5));
    println!("{}", Solution::bitwise_complement(0));
}
