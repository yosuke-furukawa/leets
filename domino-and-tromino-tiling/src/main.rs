impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let modulo = 1_000_000_007;
        if n <= 2 {
            return n;
        }

        let mut f_current: u64 = 5;
        let mut f_previous = 2;
        let mut f_before_previous = 1;
        for _ in 4..n + 1 {
            let tmp = f_previous;
            f_previous = f_current;
            f_current = (2 * f_current + f_before_previous) % modulo;
            f_before_previous = tmp;
        }
        (f_current % modulo) as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::num_tilings(3));
}
