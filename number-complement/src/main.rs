impl Solution {
    pub fn find_complement(num: i32) -> i32 {
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
    println!("{}", Solution::find_complement(5));
    println!("{}", Solution::find_complement(2));
    println!("{}", Solution::find_complement(1));
}
