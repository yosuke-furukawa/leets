impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let t = minutes_to_test / minutes_to_die;
        let mut x = 0;
        while (t + 1).pow(x) < buckets {
            x += 1;
        }
        x as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::poor_pigs(1000, 15, 60));
}
