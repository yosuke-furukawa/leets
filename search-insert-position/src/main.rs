impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::search_insert(vec![1, 3, 5, 6], 5));
    println!("{}", Solution::search_insert(vec![1, 3, 5, 6], 2));
    println!("{}", Solution::search_insert(vec![1, 3, 5, 6], 7));
    println!("{}", Solution::search_insert(vec![1, 3, 5, 6], 0));
    println!("{}", Solution::search_insert(vec![1], 0));
}
