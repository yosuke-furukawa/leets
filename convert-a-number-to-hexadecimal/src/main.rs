impl Solution {
    pub fn to_hex(num: i32) -> String {
        format!("{:x}", num)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::to_hex(26));
}
