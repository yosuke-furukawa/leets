impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return k;
        }

        let mut same = k;
        let mut diff = k * (k - 1);
        for _ in 3..n + 1 {
            let s = diff;
            let d = (same + diff) * (k - 1);
            same = s;
            diff = d;
        }

        same + diff
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::num_ways(3, 2));
}
