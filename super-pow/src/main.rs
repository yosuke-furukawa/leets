impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let mut res: i128 = 1;
        let a = (a % 1337) as i128;
        for x in b {
            res = ((res.pow(10) % 1337) * (a.pow(x as u32) % 1337) as i128) % 1337;
        }
        res as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::super_pow(2, vec![3]));
    println!("{}", Solution::super_pow(1, vec![4, 3, 3, 8, 5, 2]));
    println!("{}", Solution::super_pow(2, vec![1, 0]));
}
