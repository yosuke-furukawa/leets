impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut remainders = vec![0; 60];
        let mut count = 0;
        for t in time {
            if t % 60 == 0 {
                count += remainders[0];
            } else {
                count += remainders[60 - t as usize % 60];
            }
            remainders[t as usize % 60] += 1;
        }
        count
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::num_pairs_divisible_by60(vec![30, 20, 150, 100, 40])
    );
}
