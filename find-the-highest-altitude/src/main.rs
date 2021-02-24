impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut result = vec![0];
        let mut prev = 0;
        for g in gain {
            prev += g;
            result.push(prev);
        }
        result.iter().fold(0, |acc, n| acc.max(*n))
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::largest_altitude(vec![-5, 1, 5, 0, -7]));
    println!(
        "{}",
        Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2])
    );
}
