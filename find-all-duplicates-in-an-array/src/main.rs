impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for i in 0..nums.len() {
            let index = (nums[i].abs() - 1) as usize;
            if nums[index] < 0 {
                res.push(index as i32 + 1);
                continue;
            }
            nums[index] *= -1;
        }
        res
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1])
    );
}
