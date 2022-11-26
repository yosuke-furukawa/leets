impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let m = a.min(b);
        let mut res = 0;
        for i in 1..=m {
            if a % i == 0 && b % i == 0 {
                res += 1;
            }
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::common_factors(12, 6));
    println!("{}", Solution::common_factors(25, 30));
    println!("{}", Solution::common_factors(12, 18));
}
