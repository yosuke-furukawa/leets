impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut visited = vec![false; nums.len()];
        let mut max = 0;
        for (i, _) in nums.iter().enumerate() {
            if !visited[i] {
                let mut start = nums[i];
                let mut count = 0;
                start = nums[start as usize];
                count += 1;
                visited[start as usize] = true;
                while start != nums[i] {
                    start = nums[start as usize];
                    count += 1;
                    visited[start as usize] = true;
                }
                max = max.max(count);
            }
        }
        max
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]));
}
