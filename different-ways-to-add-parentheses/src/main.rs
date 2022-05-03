impl Solution {
    fn helper(input: &str) -> Vec<i32> {
        if input.is_empty() {
            return vec![];
        }
        if let Ok(digit) = input.parse::<i32>() {
            return vec![digit];
        }
        let mut res = vec![];
        for (i, ch) in input.chars().enumerate() {
            if ch == '+' || ch == '-' || ch == '*' {
                let left = Self::helper(&input[..i]);
                let right = Self::helper(&input[i + 1..]);
                for &a in left.iter() {
                    for &b in right.iter() {
                        match ch {
                            '+' => res.push(a + b),
                            '-' => res.push(a - b),
                            '*' => res.push(a * b),
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
        res
    }
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        Self::helper(expression.as_str())
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::diff_ways_to_compute("2-1-1".to_string()));
    println!(
        "{:?}",
        Solution::diff_ways_to_compute("2*3-4*5".to_string())
    );
}
