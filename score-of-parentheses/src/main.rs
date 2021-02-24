impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut lefts = 0;
        let mut result = 0;
        let mut skip = false;
        for c in s.chars() {
            match c {
                '(' => {
                    lefts += 1;
                    skip = false;
                }
                ')' => {
                    if !skip {
                        result += 2i32.pow(lefts - 1);
                    }
                    lefts -= 1;
                    skip = true;
                }
                _ => panic!("no such chars"),
            }
        }

        result
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::score_of_parentheses("()()".to_string()));
    println!("{}", Solution::score_of_parentheses("(()())".to_string()));
    println!("{}", Solution::score_of_parentheses("(())".to_string()));
    println!("{}", Solution::score_of_parentheses("(()(()))".to_string()));
    println!(
        "{}",
        Solution::score_of_parentheses("(()()(()(()(())()())()())()()()())".to_string())
    );
}
