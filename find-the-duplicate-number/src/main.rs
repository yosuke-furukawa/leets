impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = nums[0];
        let mut fast = nums[nums[0] as usize];
        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
        }
        slow = 0;
        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[fast as usize];
        }
        slow
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find_duplicate(vec![1, 3, 4, 2, 2]));
}
