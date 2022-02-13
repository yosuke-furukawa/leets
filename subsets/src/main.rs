impl Solution {
    fn helper(index: usize, current: Vec<i32>, nums: &[i32], results: &mut Vec<Vec<i32>>) {
        results.push(current.clone());
        for i in index..nums.len() {
            let mut current_clone = current.clone();
            current_clone.push(nums[i]);
            Solution::helper(i + 1, current_clone, nums, results);
        }
    }
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = vec![];
        Self::helper(0, vec![], &nums, &mut results);
        results
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::subsets(vec![1, 2, 3]));
    println!("{:?}", Solution::subsets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));
}
