impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut res = 0;
        for c in column_title.chars() {
            res *= 26;
            res += (c as u8 - b'A' + 1) as i32;
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::title_to_number("A".to_string()));
    println!("{}", Solution::title_to_number("AB".to_string()));
    println!("{}", Solution::title_to_number("ZY".to_string()));
}
