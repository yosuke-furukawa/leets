impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        while lo <= hi {
            let mid = (lo + hi) / 2;
            if mid % 2 == 0 {
                if mid > 0 && nums[mid - 1] == nums[mid] {
                    hi = mid - 1;
                } else if mid < nums.len() - 1 && nums[mid + 1] == nums[mid] {
                    lo = mid;
                } else {
                    return nums[mid];
                }
            } else {
                if mid > 0 && nums[mid - 1] == nums[mid] {
                    lo = mid + 1;
                } else if mid < nums.len() - 1 && nums[mid + 1] == nums[mid] {
                    hi = mid;
                } else {
                    return nums[mid];
                }
            }
        }
        nums[(lo + hi) / 2]
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
