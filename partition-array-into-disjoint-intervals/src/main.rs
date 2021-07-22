impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut max_array = vec![];
        let mut max = 0;
        for n in nums.iter() {
            max = max.max(*n);
            max_array.push(max);
        }

        let mut min_array = vec![];
        let mut min = 1_000_000_000;
        for n in nums.iter().rev() {
            min = min.min(*n);
            min_array.push(min);
        }

        for i in 1..nums.len() {
            if max_array[i - 1] <= min_array[nums.len() - i - 1] {
                return i as i32;
            }
        }
        -1
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::partition_disjoint(vec![5, 0, 3, 8, 6]));
    println!("{}", Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]));
    println!("{}", Solution::partition_disjoint(vec![1, 1, 1, 5, 6, 12]));
}
