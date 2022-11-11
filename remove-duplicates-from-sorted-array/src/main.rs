impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        for j in 1..nums.len() {
            if nums[i] != nums[j] {
                i += 1;
                nums[i] = nums[j];
            }
        }
        (i + 1) as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::remove_duplicates(&mut vec![1, 1, 2]));
}
