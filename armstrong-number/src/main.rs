impl Solution {
    pub fn is_armstrong(n: i32) -> bool {
        let mut v = vec![];
        let mut num = n;
        while num > 0 {
            v.push(num % 10);
            num /= 10;
        }
        let len = v.len() as u32;

        n == v.iter().map(|x| x.pow(len)).fold(0, |acc, x| acc + x)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::is_armstrong(153));
    println!("{}", Solution::is_armstrong(123));
}
