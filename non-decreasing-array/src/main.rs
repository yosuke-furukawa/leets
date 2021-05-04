impl Solution {
    fn check_possibility(nums: Vec<i32>) -> bool {
        let mut found = false;
        let mut current = nums[0];
        for i in 0..nums.len() - 1 {
            if current > nums[i + 1] {
                if found {
                    return false;
                }
                if i != 0 && nums[i - 1] > nums[i + 1] {
                    current = nums[i];
                } else {
                    current = nums[i + 1];
                }
                found = true;
            } else {
                current = nums[i + 1];
            }
        }
        true
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::check_possibility(vec![4, 2, 3]));
    println!("{}", Solution::check_possibility(vec![4, 2, 1]));
    println!("{}", Solution::check_possibility(vec![4, 1, 1]));
    println!("{}", Solution::check_possibility(vec![3, 4, 2, 3]));
    println!("{}", Solution::check_possibility(vec![1, 4, 1, 2]));
}
