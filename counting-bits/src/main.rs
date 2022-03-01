impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = vec![0; n as usize + 1];
        for i in 1..=n {
            res[i as usize] = res[(i & (i - 1)) as usize] + 1;
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::count_bits(2));
}
