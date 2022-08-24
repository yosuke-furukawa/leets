impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut max_len = 1;
        let mut cur_len = 1;
        for (i, num) in nums.iter().enumerate() {
            if i == 0 {
                continue;
            }
            if nums[i - 1] < *num {
                cur_len += 1;
            } else {
                cur_len = 1;
            }
            max_len = max_len.max(cur_len);
        }
        max_len
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]));
    println!("{}", Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]));
    println!("{}", Solution::find_length_of_lcis(vec![2]));
    println!("{}", Solution::find_length_of_lcis(vec![1, 3, 5, 7]));
}
