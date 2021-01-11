struct Solution {}

impl Solution {
    pub fn fixed_point(a: Vec<i32>) -> i32 {
        for (i, v) in a.iter().enumerate() {
            if *v == i as i32 {
                return *v;
            }
        }
        -1
    }
}

fn main() {
    println!("{}", Solution::fixed_point(vec![-10, -5, 0, 3, 7]));
    println!("{}", Solution::fixed_point(vec![0, 1, 2, 3, 4]));
    println!("{}", Solution::fixed_point(vec![-10, -5, 3, 4, 7, 9]));
}
