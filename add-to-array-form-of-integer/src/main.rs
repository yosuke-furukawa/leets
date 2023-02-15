impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut n = 0;
        for &x in &num {
            n = n * 10 + x;
        }
        n += k;
        let mut res = Vec::new();
        if n == 0 {
            res.push(0);
        }
        while n > 0 {
            res.push(n % 10);
            n /= 10;
        }
        res.reverse();
        res
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::add_to_array_form(vec![1, 2, 0, 0], 34));
}
