impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut res = 0;
        let mut n = n;
        let mut i = 2;
        while i <= n {
            while n % i == 0 {
                res += i;
                n /= i;
            }
            i += 1;
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_steps(3));
}
