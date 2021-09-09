impl Solution {
    fn trib_inner(n: i32, dp: &mut Vec<i32>) -> i32 {
        if let Some(v) = dp.get(n as usize) {
            return *v;
        }
        let ans =
            Self::trib_inner(n - 1, dp) + Self::trib_inner(n - 2, dp) + Self::trib_inner(n - 3, dp);
        dp.push(ans);
        ans
    }
    pub fn tribonacci(n: i32) -> i32 {
        let mut dp = vec![0, 1, 1];
        Self::trib_inner(n, &mut dp)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::tribonacci(4));
    println!("{}", Solution::tribonacci(5));
    println!("{}", Solution::tribonacci(25));
}
