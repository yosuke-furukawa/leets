impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut low: i32 = 0;
        let mut high: i32 = nums.len() as i32 - 1;
        while low <= high {
            let mid = low + (high - low) / 2;
            if mid < 0 || mid >= nums.len() as i32 {
                break;
            }
            if nums[mid as usize] == target {
                low = mid;
                high = mid;
                while low >= 0 && nums[low as usize] == target {
                    if nums[low as usize] == target {
                        low -= 1;
                    }
                }
                while high < nums.len() as i32 && nums[high as usize] == target {
                    if nums[high as usize] == target {
                        high += 1;
                    }
                }
                return vec![low as i32 + 1, high as i32 - 1];
            }
            if nums[mid as usize] < target {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        vec![-1, -1]
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8));
    println!("{:?}", Solution::search_range(vec![], 0));
    println!("{:?}", Solution::search_range(vec![1], 0));
    println!("{:?}", Solution::search_range(vec![1], 1));
    println!("{:?}", Solution::search_range(vec![2, 2], 2));
}
