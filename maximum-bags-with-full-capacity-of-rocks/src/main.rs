impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut capacity = capacity;
        for (i, rock) in rocks.iter().enumerate() {
            capacity[i] -= rock;
        }
        capacity.sort_unstable();
        let mut additional_rocks = additional_rocks;
        let mut res = 0;
        for &cap in &capacity {
            if cap >= 0 && additional_rocks - cap >= 0 {
                additional_rocks -= cap;
                res += 1;
            }
            if additional_rocks == 0 {
                return res;
            }
        }
        res
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::maximum_bags(vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2)
    );
    println!(
        "{}",
        Solution::maximum_bags(vec![91,54,63,99,24,45,78], vec![35,32,45,98,6,1,25], 17)
    );
}
