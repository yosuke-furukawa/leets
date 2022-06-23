impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut res = 0;
        let mut n = n as i64;
        while n != 1 {
            if n == 3 {
                res += 2;
                n = 1;
            } else if n % 2 == 0 {
                n /= 2;
                res += 1;
            } else if n % 4 == 1 {
                n -= 1;
                res += 1;
            } else {
                n += 1;
                res += 1;
            }
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::integer_replacement(100000));
}
