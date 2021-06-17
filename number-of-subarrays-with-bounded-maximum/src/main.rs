impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut count = 0;
        let mut window = 0;
        while j < nums.len() {
            if left <= nums[j] && right >= nums[j] {
                window = j - i + 1;
            } else if nums[j] > right {
                window = 0;
                i = j + 1;
            }
            count += window;
            j += 1;
        }
        count as i32
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::num_subarray_bounded_max(vec![2, 1, 4, 3], 2, 3)
    );
    println!(
        "{}",
        Solution::num_subarray_bounded_max(
            vec![2, 1, 4, 3, 11, 2, 23, 3, 4, 3, 2, 1, 1, 2, 3, 4, 5, 6, 7, 2, 9],
            1,
            5
        )
    );
}
