use std::collections::HashMap;

impl Solution {
    fn parse(expressions: &mut Vec<&str>, variables: HashMap<String, i32>) -> Option<i32> {
        let mut variables = variables;
        let mut result = None;
        while let Some(expression) = expressions.pop() {
            match expression {
                ")" => {
                    return result;
                }
                "(" => {
                    return Self::parse(expressions, variables.clone());
                }
                "add" => {
                    let a = Self::parse(expressions, variables.clone()).unwrap();
                    let b = Self::parse(expressions, variables.clone()).unwrap();
                    result = Some(a + b);
                }
                "mult" => {
                    let a = Self::parse(expressions, variables.clone()).unwrap();
                    let b = Self::parse(expressions, variables.clone()).unwrap();
                    result = Some(a * b);
                }
                "let" => {
                    let mut name = None;
                    let mut let_result = None;
                    while ")" != expressions[expressions.len() - 1] {
                        if name.is_none() {
                            name = Some(expressions[expressions.len() - 1]);
                            let_result = Self::parse(expressions, variables.clone());
                            continue;
                        }
                        if name.is_some() {
                            if expressions[expressions.len() - 1] == ")" {
                                break;
                            }
                            if let Some(val) = Self::parse(expressions, variables.clone()) {
                                variables.insert(name.unwrap().to_string(), val);
                                name = None;
                                let_result = None;
                                continue;
                            } else {
                                break;
                            }
                        }
                    }
                    result = let_result;
                }
                c if c.parse::<i32>().is_ok() => {
                    return Some(c.parse::<i32>().unwrap());
                }
                _ => {
                    return variables.get(expression).cloned();
                }
            }
        }
        None
    }
    pub fn evaluate(expression: String) -> i32 {
        let mut expression = expression;
        expression = expression.replace("(", " ( ");
        expression = expression.replace(")", " ) ");

        Self::parse(
            &mut expression.split_whitespace().rev().collect::<Vec<&str>>(),
            HashMap::new(),
        )
        .unwrap()
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::evaluate("(let x 2 (mult x (let x 3 y 4 (add x y))))".to_string())
    );
    println!("{}", Solution::evaluate("(let x 3 x 2 x)".to_string()));
    println!(
        "{}",
        Solution::evaluate("(let x 1 y 2 x (add x y) (add x y))".to_string())
    );
}
