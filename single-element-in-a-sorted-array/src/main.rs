impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |result, n| result ^ n)
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8])
    );
    println!(
        "{}",
        Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11])
    );
    println!(
        "{}",
        Solution::single_non_duplicate(vec![1, 1, 2, 2, 3, 3, 4, 4, 8, 8, 9])
    );
}
