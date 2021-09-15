impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        if arr.len() < 2 {
            return 1;
        }
        if arr.len() == 2 {
            if arr[0] != arr[1] {
                return 2;
            } else {
                return 1;
            }
        }
        fn helper(index: usize, arr: &[i32]) -> i32 {
            let mut count = 2;
            for i in index..arr.len() {
                if arr[i - 2] == arr[i - 1] && arr[i - 1] == arr[i] {
                    return 1;
                }
                if arr[i - 2] > arr[i - 1] && arr[i - 1] < arr[i]
                    || arr[i - 2] < arr[i - 1] && arr[i - 1] > arr[i]
                {
                    count += 1;
                } else {
                    break;
                }
            }
            count
        }
        let mut index = 2;
        let mut max = 0;
        while index < arr.len() {
            let count = helper(index, &arr);
            max = max.max(count);
            if count == 1 {
                index += 1;
                continue;
            }
            index += count as usize - 1;
        }
        max
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9])
    );
    println!("{}", Solution::max_turbulence_size(vec![4, 8, 12, 16]));
    println!("{}", Solution::max_turbulence_size(vec![100]));
    println!("{}", Solution::max_turbulence_size(vec![100, 100, 100]));
}
