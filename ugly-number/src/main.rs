impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        let mut n = n;
        if n <= 0 {
            return false;
        }
        while n > 1 {
            if n % 2 == 0 {
                n /= 2;
            } else if n % 3 == 0 {
                n /= 3;
            } else if n % 5 == 0 {
                n /= 5;
            } else {
                return false;
            }
        }
        true
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::is_ugly(6));
    println!("{}", Solution::is_ugly(1));
    println!("{}", Solution::is_ugly(14));
    println!("{}", Solution::is_ugly(-2147483648));
}
