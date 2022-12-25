impl Solution {
    fn binary_search(nums: &Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;
        while l < r {
            let m = (l + r) / 2;
            if nums[m as usize] == target {
                return m + 1;
            } else if nums[m as usize] < target {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        if nums[l as usize] > target {
            l
        } else {
            l + 1
        }
    }
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        let mut res = vec![0; queries.len()];
        for i in 0..queries.len() {
            let q = queries[i];
            let idx = Self::binary_search(&nums, q);
            res[i] = idx as i32;
        }
        res
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21])
    );
    println!("{:?}", Solution::answer_queries(vec![2, 3, 4, 5], vec![1]));
    println!(
        "{:?}",
        Solution::answer_queries(
            vec![432287, 176328, 634565, 651053, 768487],
            vec![647523, 114549, 693419, 570141, 990359, 889170]
        )
    );
}
