use std::collections::HashMap;

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        let mut dp: Vec<i64> = vec![1; arr.len()];
        let mut calcs: HashMap<i32, i32> = HashMap::new();
        arr.sort_unstable();
        let mut count = 0;
        for i in 0..arr.len() {
            calcs.insert(arr[i], i as i32);
            for j in 0..i {
                if arr[i] % arr[j] == 0 {
                    if let Some(&k) = calcs.get(&(arr[i] / arr[j])) {
                        dp[i] += dp[j] * dp[k as usize];
                    }
                }
            }
            count = (count + dp[i]) % (10i64.pow(9) + 7);
        }
        count as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::num_factored_binary_trees(vec![2, 4, 8, 16]));
    println!("{}", Solution::num_factored_binary_trees(vec![2, 4, 5, 10]));
    println!(
        "{}",
        Solution::num_factored_binary_trees(vec![2, 3, 4, 5, 6, 7, 8, 9])
    );
    println!(
        "{}",
        Solution::num_factored_binary_trees(vec![2, 3, 4, 8, 9, 27, 81, 128])
    );
}
