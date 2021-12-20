impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort_unstable();
        let mut diff = arr[1] - arr[0];
        for nums in arr.windows(2) {
            let a = nums[0];
            let b = nums[1];
            diff = diff.min(b - a);
        }
        let mut results = vec![];
        for nums in arr.windows(2) {
            let a = nums[0];
            let b = nums[1];
            if diff == b - a {
                results.push(vec![a, b]);
            }
        }
        results
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::minimum_abs_difference(vec![4, 2, 1, 3]));
}
