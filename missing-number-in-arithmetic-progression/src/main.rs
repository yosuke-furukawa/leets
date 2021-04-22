impl Solution {
    pub fn missing_number(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let s1 = ((n + 1) * (arr[0] + arr[n - 1]) as usize) as i32 / 2;
        let s2: i32 = arr.iter().sum();
        s1 - s2
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::missing_number(vec![5, 7, 11, 13]));
}
