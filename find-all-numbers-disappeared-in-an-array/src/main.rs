impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut results = (1..=nums.len() as i32).collect::<Vec<i32>>();
        for n in nums {
            results[n as usize - 1] = -1;
        }
        results.into_iter().filter(|&x| x >= 0).collect()
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1])
    );
}
