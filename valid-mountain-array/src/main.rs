impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        if arr[0] > arr[1] {
            return false;
        }
        let mut is_increasing = true;
        for i in 0..arr.len() - 1 {
            if is_increasing && arr[i] > arr[i + 1] {
                is_increasing = false;
            }
            if !is_increasing && arr[i] <= arr[i + 1] {
                return false;
            }
            if is_increasing && arr[i] >= arr[i + 1] {
                return false;
            }
        }
        !is_increasing
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::valid_mountain_array(vec![2, 1]));
    println!("{}", Solution::valid_mountain_array(vec![3, 5, 5]));
    println!("{}", Solution::valid_mountain_array(vec![0, 3, 2, 1]));
    println!(
        "{}",
        Solution::valid_mountain_array(vec![0, 2, 3, 4, 5, 2, 1, 0])
    );
    println!(
        "{}",
        Solution::valid_mountain_array(vec![0, 2, 3, 3, 5, 2, 1, 0])
    );
    println!(
        "{}",
        Solution::valid_mountain_array(vec![0, 2, 3, 4, 5, 2, 2, 0])
    );
    println!(
        "{}",
        Solution::valid_mountain_array(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    );
}
