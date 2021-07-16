use std::collections::HashSet;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut set = HashSet::new();
        let mut results = vec![];

        for s1 in 0..nums.len() {
            for e1 in (s1 + 1..nums.len()).rev() {
                let a = nums[s1];
                let b = nums[e1];
                let mut s2 = s1 + 1;
                let mut e2 = e1 - 1;
                while s2 < e2 {
                    let c = nums[s2];
                    let d = nums[e2];
                    if a + b + c + d > target {
                        e2 -= 1;
                    } else if a + b + c + d < target {
                        s2 += 1;
                    } else {
                        let answer = [a, b, c, d].to_vec();
                        if !set.contains(&answer) {
                            results.push(answer.clone());
                            set.insert(answer);
                        }
                        if c + d > 0 {
                            e2 -= 1;
                        } else {
                            s2 += 1;
                        }
                    }
                }
            }
        }
        results
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0));
}
