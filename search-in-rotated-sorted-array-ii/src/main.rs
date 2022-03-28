impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = (left + right) / 2;

            if nums[mid as usize] == target {
                return true;
            }

            if nums[left as usize] == nums[mid as usize]
                && nums[right as usize] == nums[mid as usize]
            {
                left += 1;
                right -= 1;
            } else if nums[left as usize] <= nums[mid as usize] {
                if nums[left as usize] <= target && target < nums[mid as usize] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                if nums[mid as usize] < target && target <= nums[right as usize] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        false
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    println!("{}", Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
}
