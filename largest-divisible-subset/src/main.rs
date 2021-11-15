impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut divs = vec![1; nums.len()];
        let mut prev = vec![-1; nums.len()];
        let mut max_index = 0;
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && divs[i] < divs[j] + 1 {
                    prev[i] = j as i32;
                    divs[i] = divs[j] + 1;
                }
            }

            if divs[i] > divs[max_index as usize] {
                max_index = i as i32;
            }
        }
        let mut k = max_index;
        let mut result = vec![];
        while k >= 0 {
            result.push(nums[k as usize]);
            k = prev[k as usize];
        }
        result
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::largest_divisible_subset(vec![1, 2, 3]));
    println!(
        "{:?}",
        Solution::largest_divisible_subset(vec![3, 4, 16, 8])
    );
}
