impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo = vec![1, 1, 2];
        for i in 3..=n {
            memo.push(memo[i as usize - 1] + memo[i as usize - 2]);
        }
        memo[n as usize]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::climb_stairs(10));
}
