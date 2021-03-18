impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut pn = 0;
        for n in nums.as_slice().windows(2) {
            match (pn, n[0], n[1]) {
                (_, _, _) if pn > 0 && n[0] < n[1] => {
                    count += 1;
                    pn = -1;
                }
                (_, _, _) if pn < 0 && n[0] > n[1] => {
                    count += 1;
                    pn = 1;
                }
                (_, _, _) if pn == 0 && n[0] != n[1] => {
                    count += 1;
                    pn = n[0] - n[1];
                }
                _ => continue,
            }
        }
        count
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]));
    println!(
        "{}",
        Solution::wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8])
    );
    println!(
        "{}",
        Solution::wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    );
    println!("{}", Solution::wiggle_max_length(vec![3, 3, 3, 2, 5]));
}
