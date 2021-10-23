impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut min = nums[0];
        for n in nums {
            if min > n {
                min = n;
                break;
            }
        }

        min
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find_min(vec![1, 3, 5]));
}
