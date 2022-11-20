impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    if nums[i] != nums[j] && nums[j] != nums[k] && nums[i] != nums[k] {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::unequal_triplets(vec![4, 4, 2, 4, 3]));
}
