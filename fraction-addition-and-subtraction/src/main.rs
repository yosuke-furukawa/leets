use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
struct Fraction {
    sign: i32,
    num: i32,
    denom: i32,
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.sign * self.num, self.denom)
    }
}

impl Fraction {
    fn new(sign: i32, num: i32, denom: i32) -> Self {
        let gcd = Self::gcd(num, denom);
        Fraction {
            sign,
            num: num / gcd,
            denom: denom / gcd,
        }
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    fn lcm(a: i32, b: i32) -> i32 {
        a * b / Self::gcd(a, b)
    }

    fn add(&self, other: &Self) -> Self {
        let lcm = Self::lcm(self.denom, other.denom);
        let num = self.sign * self.num * (lcm / self.denom)
            + other.sign * other.num * (lcm / other.denom);
        let sign = if num < 0 { -1 } else { 1 };
        Self::new(sign, num.abs(), lcm)
    }
}

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let chars: Vec<char> = expression.chars().collect();
        let mut fractions = vec![];
        let mut cursor = 0;
        while cursor < chars.len() {
            if chars[cursor] == '/' {
                let mut i = cursor;
                while i > 0 && chars[i - 1].is_digit(10) {
                    i -= 1;
                }
                let num = chars[i..cursor]
                    .iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                let mut j = cursor + 1;
                while j + 1 < chars.len() && chars[j + 1].is_digit(10) {
                    j += 1;
                }
                let denom = chars[cursor + 1..=j]
                    .iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                let sign = if i > 0 && chars[i - 1] == '-' { -1 } else { 1 };
                let fraction = Fraction::new(sign, num, denom);
                fractions.push(fraction);
            }
            cursor += 1;
        }
        fractions
            .iter()
            .fold(Fraction::new(1, 0, 1), |acc, x| acc.add(x))
            .to_string()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::fraction_addition("-1/2+1/2".to_string()));
    println!(
        "{}",
        Solution::fraction_addition("-1/2+1/2+1/3".to_string())
    );
    println!("{}", Solution::fraction_addition("1/3-1/2".to_string()));
    println!(
        "{}",
        Solution::fraction_addition("-5/2+10/3+7/9".to_string())
    );
}
