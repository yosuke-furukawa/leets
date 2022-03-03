impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        if a.len() < 3 {
            return 0;
        }
        let mut res = 0;
        let mut count = 0;
        for i in 2..a.len() {
            if a[i] - a[i - 1] == a[i - 1] - a[i - 2] {
                count += 1;
            } else {
                res += count * (count + 1) / 2;
                count = 0;
            }
        }
        res + count * (count + 1) / 2
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4])
    );
    println!(
        "{}",
        Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4, 5])
    );
    println!(
        "{}",
        Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4, 6, 7, 8])
    );
    println!(
        "{}",
        Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4, 6, 7, 8])
    );
    println!(
        "{}",
        Solution::number_of_arithmetic_slices(vec![3, -1, -5, -9])
    );
}
