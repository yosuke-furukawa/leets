impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut slow = 0;
        let mut fast = 0;
        let mut k = k;
        while k > 0 {
            if fast == arr.len() {
                return slow + k;
            }
            if arr[fast] == slow + 1 {
                fast += 1;
            } else {
                k -= 1;
            }
            slow += 1;
        }
        slow
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5));
}
