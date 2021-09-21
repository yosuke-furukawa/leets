impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut max = 0;
        for n in nums {
            match n {
                1 => {
                    count += 1;
                    max = max.max(count);
                }
                _ => {
                    count = 0;
                }
            }
        }
        max
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1])
    );
}
