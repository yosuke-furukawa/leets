impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let modulo = 1_000_000_007;
        let l = a / gcd(a, b) * b;
        let m = l / a + l / b - 1;
        let q = n / m;
        let r = n % m;
        let mut ans: u64 = q as u64 * l as u64 % modulo;
        if r == 0 {
            return ans as i32;
        }

        let mut heads = vec![a as u64, b as u64];
        for _ in 0..r - 1 {
            if heads[0] <= heads[1] {
                heads[0] += a as u64;
            } else {
                heads[1] += b as u64;
            }
        }

        ans += heads[0].min(heads[1]);
        (ans % modulo) as i32
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::nth_magical_number(1, 2, 3));
    println!("{}", Solution::nth_magical_number(4, 2, 3));
    println!("{}", Solution::nth_magical_number(5, 2, 4));
    println!("{}", Solution::nth_magical_number(3, 6, 4));
}
