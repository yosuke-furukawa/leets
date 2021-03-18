impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        nums.as_slice()
            .windows(2)
            .fold((1, 0), |(count, pn), n| {
                let diff = n[1] - n[0];
                if diff == 0 || (pn as i32).signum() == diff.signum() {
                    (count, pn)
                } else {
                    (count + 1, diff)
                }
            })
            .0
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]));
    println!(
        "{}",
        Solution::wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8])
    );
    println!(
        "{}",
        Solution::wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    );
    println!("{}", Solution::wiggle_max_length(vec![3, 3, 3, 2, 5]));
}
