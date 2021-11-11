impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut min_sum = 0;
        let mut sum = 0;
        for n in nums {
            sum += n;
            if min_sum > sum {
                min_sum = sum;
            }
        }
        -min_sum + 1
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_start_value(vec![-3, 2, -3, 4, 2]));
    println!("{}", Solution::min_start_value(vec![1, 2]));
    println!("{}", Solution::min_start_value(vec![1, -2, -3]));
}
