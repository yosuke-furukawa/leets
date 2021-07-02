impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        if arr.is_empty() {
            return arr;
        }
        let mut left = 0;
        let mut right = arr.len() - k as usize;
        while left < right {
            let mid = (left + right) / 2;
            if x - arr[mid] > arr[mid + k as usize] - x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        let mut result = vec![];
        for n in arr.iter().skip(left).take(k as usize) {
            result.push(*n);
        }
        result
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3)
    );
}
