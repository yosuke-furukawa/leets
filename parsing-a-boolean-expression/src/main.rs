impl Solution {
    fn parse(expressions: &mut std::str::Chars) -> bool {
        match expressions.next() {
            Some('t') => true,
            Some('f') => false,
            Some('!') => Self::parse_not(expressions),
            Some('&') => Self::parse_and(expressions),
            Some('|') => Self::parse_or(expressions),
            _ => unreachable!(),
        }
    }
    fn parse_not(expressions: &mut std::str::Chars) -> bool {
        let mut result = true;
        loop {
            match expressions.next() {
                Some('(') => {
                    result = Self::parse(expressions);
                }
                Some(')') => {
                    return !result;
                }
                _ => unreachable!(),
            }
        }
    }
    fn parse_and(expressions: &mut std::str::Chars) -> bool {
        let mut result = true;
        loop {
            match expressions.next() {
                Some('(') => {
                    result = Self::parse(expressions);
                }
                Some(',') => {
                    result = Self::parse(expressions) && result;
                }
                Some(')') => {
                    return result;
                }
                _ => unreachable!(),
            }
        }
    }
    fn parse_or(expressions: &mut std::str::Chars) -> bool {
        let mut result = false;
        loop {
            match expressions.next() {
                Some('(') => {
                    result = Self::parse(expressions);
                }
                Some(',') => {
                    result = Self::parse(expressions) || result;
                }
                Some(')') => {
                    return result;
                }
                _ => unreachable!(),
            }
        }
    }
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut tokens = expression.chars();
        Self::parse(&mut tokens)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::parse_bool_expr("!(f)".to_string()));
    println!("{}", Solution::parse_bool_expr("|(f,t)".to_string()));
    println!("{}", Solution::parse_bool_expr("&(t,f)".to_string()));
    println!("{}", Solution::parse_bool_expr("&(f,|(f,f))".to_string()));
    println!(
        "{}",
        Solution::parse_bool_expr("|(&(t,f,t),!(t))".to_string())
    );
    println!("{}", Solution::parse_bool_expr("&(t,f,t)".to_string()));
    println!("{}", Solution::parse_bool_expr("|(t,t,f)".to_string()));
}
