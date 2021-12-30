impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut r = 0;
        for n in 1..=k {
            r = (r * 10 + 1) % k;
            if r == 0 {
                return n;
            }
        }
        -1
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::smallest_repunit_div_by_k(1));
    println!("{}", Solution::smallest_repunit_div_by_k(2));
    println!("{}", Solution::smallest_repunit_div_by_k(3));
}
