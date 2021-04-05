impl Solution {
    pub fn is_ideal_permutation(a: Vec<i32>) -> bool {
        for (i, n) in a.iter().enumerate() {
            if (i as i32 - n).abs() > 1 {
                return false;
            }
        }
        true
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::is_ideal_permutation(vec![1,0,2]));
}
