struct Solution {}

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        let mut nums: Vec<i32> = Vec::with_capacity((n+1) as usize);
        for i in 0..n+1 {
            nums.push(match i {
                0 => 0,
                1 => 1,
                i if i % 2 == 0 => nums[(i / 2) as usize],
                _ => nums[(i / 2) as usize] + nums[(i / 2 + 1) as usize],
            });
        }
        *nums.iter().max().unwrap()
    }
}

fn main() {
    println!("{}", Solution::get_maximum_generated(100));
}
