use std::collections::HashSet;

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut uglies = vec![1];
        let mut set = HashSet::new();
        let mut index = vec![0; primes.len()];
        while uglies.len() < n as usize {
            let mut min = std::i32::MAX;
            let mut min_index = 0;
            for i in 0..primes.len() {
                let ugly = uglies[index[i]] * primes[i];
                if ugly < min {
                    min = ugly;
                    min_index = i;
                }
            }
            if set.contains(&min) {
                index[min_index] += 1;
                continue;
            }
            set.insert(min);
            uglies.push(min);
            index[min_index] += 1;
        }
        uglies[n as usize - 1]
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::nth_super_ugly_number(12, vec![2, 7, 13, 19])
    );
    println!("{}", Solution::nth_super_ugly_number(1, vec![2, 3, 5]));
}
