impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, n) in numbers.iter().enumerate() {
            for (j, m) in numbers.iter().enumerate().rev() {
                if i >= j {
                    break;
                }
                if n + m == target {
                    return vec![i as i32 + 1, j as i32 + 1];
                }
            }
        }
        vec![]
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}
