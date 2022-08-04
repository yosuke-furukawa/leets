impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        if p == q {
            return 1;
        }
        let l = lcm(p, q);
        let w = l / q;
        let h = l / p;
        match (w % 2, h % 2) {
            (0, 0) => 0,
            (0, _) => 2,
            (_, 0) => 0,
            (_, _) => 1,
        }
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::mirror_reflection(2, 1));
    println!("{}", Solution::mirror_reflection(3, 1));
}
