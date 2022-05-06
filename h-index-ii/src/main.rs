impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len() as i32;
        let mut h = 0;
        let mut l = 0;
        let mut r = n - 1;
        while l <= r {
            let m = (l + r) / 2;
            if citations[m as usize] >= n - m {
                h = n - m;
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
        h as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::h_index(vec![0, 1, 3, 5, 6]));
    println!("{}", Solution::h_index(vec![1]));
}
