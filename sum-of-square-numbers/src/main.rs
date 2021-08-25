impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let sqrt = (c as f64).sqrt() as i32;
        for i in 0..=sqrt {
            let temp: f64 = ((c - i * i) as f64).sqrt();
            if (temp.trunc() - temp).abs() < f64::EPSILON {
                return true;
            }
        }
        false
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::judge_square_sum(5));
    println!("{}", Solution::judge_square_sum(3));
    println!("{}", Solution::judge_square_sum(4));
    println!("{}", Solution::judge_square_sum(2));
    println!("{}", Solution::judge_square_sum(1));
    println!("{}", Solution::judge_square_sum(1_000_000_007));
    println!("{}", Solution::judge_square_sum(2147483647));
}
