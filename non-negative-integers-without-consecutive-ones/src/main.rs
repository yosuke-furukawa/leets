impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let bit = format!("{:b}", n as usize);
        let n = bit.len();
        let mut zeros = vec![0; n];
        let mut ones = vec![0; n];
        zeros[0] = 1;
        ones[0] = 1;
        for i in 1..n {
            zeros[i] = zeros[i - 1] + ones[i - 1];
            ones[i] = zeros[i - 1];
        }
        let mut result = zeros[n - 1] + ones[n - 1];

        let chars: Vec<char> = bit.chars().rev().collect();
        for i in (0..n - 1).rev() {
            match (chars[i], chars[i + 1]) {
                ('1', '1') => break,
                ('0', '0') => result -= ones[i],
                _ => continue,
            }
        }
        result
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find_integers(1));
    println!("{}", Solution::find_integers(2));
    println!("{}", Solution::find_integers(5));
    println!("{}", Solution::find_integers(8));
    println!("{}", Solution::find_integers(12));
}
