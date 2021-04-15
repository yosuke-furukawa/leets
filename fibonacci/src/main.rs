impl Solution {
    fn fib_inner(n: i32, dp: &mut Vec<i32>) -> i32 {
        if let Some(v) = dp.get(n as usize) {
            return *v;
        }
        let ans = Self::fib_inner(n - 1, dp) + Self::fib_inner(n - 2, dp);
        dp.push(ans);
        ans
    }
    pub fn fib(n: i32) -> i32 {
        let mut dp = vec![];
        dp.push(0);
        dp.push(1);
        dp.push(1);
        dp.push(2);
        dp.push(3);
        Self::fib_inner(n, &mut dp)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::fib(2));
    println!("{}", Solution::fib(3));
    println!("{}", Solution::fib(4));
    println!("{}", Solution::fib(5));
    println!("{}", Solution::fib(6));
    println!("{}", Solution::fib(7));
}
