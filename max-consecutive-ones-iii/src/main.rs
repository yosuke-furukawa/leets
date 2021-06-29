impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut max = 0;
        let mut zero_count = 0;
        let len = nums.len();
        let mut start = 0;

        for end in 0..len {
            if nums[end] == 0 {
                zero_count += 1;
            }

            while zero_count > k {
                if nums[start] == 0 {
                    zero_count -= 1;
                }

                start += 1;
            }
            max = max.max(end - start + 1);
        }
        max as i32
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2)
    );
    println!(
        "{}",
        Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 3)
    );
    println!(
        "{}",
        Solution::longest_ones(
            vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
            3
        )
    );
}
