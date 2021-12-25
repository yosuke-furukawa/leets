impl Solution {
    pub fn calculate(mut s: String) -> i32 {
        s.push('.');
        s.chars()
            .fold((vec![], 0i32, '+'), |(mut s, mut n, mut op), c| {
                if c == ' ' {
                    return (s, n, op);
                }
                match c.to_digit(10) {
                    Some(digit) => n = n * 10 + digit as i32,
                    _ => {
                        match op {
                            '+' => s.push(n),
                            '-' => s.push(-n),
                            '/' => *s.last_mut().unwrap() /= n,
                            '*' => *s.last_mut().unwrap() *= n,
                            _ => {}
                        }
                        op = c;
                        n = 0;
                    }
                }
                (s, n, op)
            })
            .0
            .iter()
            .sum()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::calculate("3+2*2".to_string()));
}
