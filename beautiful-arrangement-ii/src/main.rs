impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut ans = vec![0; n as usize];
        let mut c = 0;
        for v in 1..n-k {
            ans[c] = v;
            c += 1;
        }
        for i in 0..=k {
            ans[c] = if i % 2 == 0 {
                n - k + i / 2 
            } else {
                n - i / 2
            };
            c += 1;
        }
        ans
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::construct_array(3, 2));
}
