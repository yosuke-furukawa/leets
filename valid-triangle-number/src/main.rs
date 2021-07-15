impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut res = 0;

        for c in 2..n {
            let mut a = 0;
            let mut b = c - 1;
            while a < b {
                if nums[a] + nums[b] > nums[c] {
                    res += b - a;
                    b -= 1;
                } else {
                    a += 1;
                }
            }
        }

        res as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::triangle_number(vec![2, 2, 3, 4]));
    println!("{}", Solution::triangle_number(vec![2, 2, 3, 4, 5]));
    println!("{}", Solution::triangle_number(vec![2, 2, 2, 4]));
    println!("{}", Solution::triangle_number(vec![1, 1, 3, 4]));
    println!(
        "{}",
        Solution::triangle_number(vec![1, 1, 3, 4, 5, 6, 7, 8])
    );
    println!(
        "{}",
        Solution::triangle_number(vec![
            55, 68, 11, 97, 3, 81, 26, 25, 15, 84, 26, 8, 36, 93, 53, 45, 19, 67, 26, 78, 0, 78,
            12, 91, 59, 7, 22, 60, 48, 73, 85, 60, 19, 37, 44, 61, 41, 10, 9, 8, 63, 60, 23, 68,
            96, 35, 98, 23, 17, 66, 72, 64, 39, 76, 27, 55, 32, 11, 80, 64, 35, 75, 12, 54, 87, 12,
            34, 15, 61, 77, 56, 39, 57, 28, 87, 4, 70, 6, 1, 21, 29, 59, 66, 32, 35, 9, 56, 63, 97,
            45, 81, 62, 70, 29, 84, 90, 78, 54, 11, 100
        ])
    );
}
