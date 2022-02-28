impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return vec![];
        }
        let mut res = vec![];
        let mut start = 0;
        let mut end = 0;
        for i in 1..nums.len() {
            if nums[i] - nums[i - 1] != 1 {
                if start == end {
                    res.push(nums[start].to_string());
                } else {
                    res.push(format!("{}->{}", nums[start], nums[end]));
                }
                start = i;
                end = i;
            } else {
                end = i;
            }
        }
        if start == end {
            res.push(nums[start].to_string());
        } else {
            res.push(format!("{}->{}", nums[start], nums[end]));
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]));
}
