impl Solution {
    fn count_smaller(m: i32, n: i32, num: i32) -> i32 {
        let mut row = m - 1;
        let mut col = 0;
        let mut count = 0;
        while row >= 0 && col < n {
            let p = (row + 1) * (col + 1);
            if p <= num {
                count += row + 1;
                col += 1;
            } else {
                row -= 1;
            }
        }
        count
    }
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let mut left = 1;
        let mut right = m * n;
        while left < right {
            let mid = left + (right - left) / 2;
            let count = Self::count_smaller(m, n, mid);
            if count >= k {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find_kth_number(3, 3, 5));
}
