impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::bulb_switch(0));
    println!("{}", Solution::bulb_switch(1));
    println!("{}", Solution::bulb_switch(3));
    println!("{}", Solution::bulb_switch(4));
    println!("{}", Solution::bulb_switch(5));
    println!("{}", Solution::bulb_switch(20));
    println!("{}", Solution::bulb_switch(1000000));
}
