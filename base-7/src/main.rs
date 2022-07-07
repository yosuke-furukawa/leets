impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let mut result = "".to_string();
        let is_minus = num < 0;
        let mut num = num.abs();
        while num > 0 {
            result = (num % 7).to_string() + result.as_str();
            num /= 7;
        }
        if is_minus {
            "-".to_string() + result.as_str()
        } else if result.is_empty() {
            "0".to_string()
        } else {
            result
        }
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::convert_to_base7(100));
}
