impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let len = 1 << n;
        let mut ans = vec![];
        for i in 0..len {
            let n = i ^ i >> 1;
            ans.push(n);
        }
        ans
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::gray_code(2));
    println!("{:?}", Solution::gray_code(3));
    println!("{:?}", Solution::gray_code(4));
    println!("{:?}", Solution::gray_code(5));
    println!("{:?}", Solution::gray_code(6));
}
