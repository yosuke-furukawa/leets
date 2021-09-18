impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut res = vec![];
        if num.is_empty() {
            return res;
        }
        fn backtrack(
            index: usize,
            val: i64,
            prev: i64,
            curr: i64,
            exp: &mut Vec<String>,
            target: i64,
            result: &mut Vec<String>,
            num: &str,
        ) {
            if index == num.len() {
                if val == target && curr == 0 {
                    result.push(exp.iter().skip(1).fold(String::new(), |acc, cur| acc + cur));
                }
                return;
            }
            let v = curr * 10 + num.get(index..index + 1).unwrap().parse::<i64>().unwrap();
            if v > 0 {
                backtrack(index + 1, val, prev, v, exp, target, result, num);
            }
            exp.push("+".to_string());
            exp.push(v.to_string());
            backtrack(index + 1, val + v, v, 0, exp, target, result, num);
            exp.pop();
            exp.pop();
            if !exp.is_empty() {
                exp.push("-".to_string());
                exp.push(v.to_string());
                backtrack(index + 1, val - v, -v, 0, exp, target, result, num);
                exp.pop();
                exp.pop();
                exp.push("*".to_string());
                exp.push(v.to_string());
                backtrack(
                    index + 1,
                    val - prev + (v * prev),
                    v * prev,
                    0,
                    exp,
                    target,
                    result,
                    num,
                );
                exp.pop();
                exp.pop();
            }
        }
        let mut exp = vec![];
        backtrack(0, 0, 0, 0, &mut exp, target as i64, &mut res, &num);
        res
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::add_operators("123".to_string(), 6));
}
