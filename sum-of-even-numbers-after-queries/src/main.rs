impl Solution {
    pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        let mut result = vec![];
        let mut sum = nums.iter().filter(|&x| x % 2 == 0).sum();
        for query in queries {
            let index = query[1] as usize;
            let val = query[0];
            match (nums[index], nums[index] + val) {
                (x, y) if x % 2 == 0 && y % 2 == 0 => sum += val,
                (x, y) if x % 2 == 0 && y % 2 != 0 => sum -= x,
                (x, y) if x % 2 != 0 && y % 2 == 0 => sum += y,
                _ => (),
            }
            nums[index] += val;
            result.push(sum);
        }
        result
    }
}

struct Solution;

macro_rules! grid {
    ( $([$( $x:expr ),*]),* ) => {
        {
            vec![
                $(
                    vec![$($x), *],
                )*
            ]
        }
    };
}

fn main() {
    println!(
        "{:?}",
        Solution::sum_even_after_queries(vec![1, 2, 3, 4], grid![[1, 0], [-3, 1], [-4, 0], [2, 3]])
    );
}
