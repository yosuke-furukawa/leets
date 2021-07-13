impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] > nums[mid + 1] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]));
}
