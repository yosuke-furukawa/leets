impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        if a.len() < 3 {
            return 0;
        }

        let mut result = 0;
        let mut count = 0;
        let mut diff = -100000000;
        for nums in a.windows(3) {
            let d1 = nums[1] - nums[0];
            let d2 = nums[2] - nums[1];
            match (diff, d1, d2) {
                (_, _, _) if diff != d1 && d1 == d2 => count += 3,
                (_, _, _) if diff == d1 && d1 == d2 => count += 1,
                (_, _, _) if diff == d1 && d1 != d2 => {
                    result += (count - 1) * (count - 2) / 2;
                    count = 0;
                }
                _ => count = 0,
            }
            diff = d1;
        }

        if count > 1 {
            result += (count - 1) * (count - 2) / 2;
        }
        result
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
