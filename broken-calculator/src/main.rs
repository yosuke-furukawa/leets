impl Solution {
    pub fn broken_calc(x: i32, y: i32) -> i32 {
        let mut y = y;
        let mut count = 0;
        while x < y {
            count += 1;
            if y % 2 == 1 {
                y += 1;
            } else {
                y /= 2;
            }
        }
        count + x - y
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::broken_calc(2, 3));
    println!("{}", Solution::broken_calc(5, 8));
    println!("{}", Solution::broken_calc(3, 10));
    println!("{}", Solution::broken_calc(1024, 1));
    println!("{}", Solution::broken_calc(432, 1000000000));
}
