impl Solution {
    pub fn interpret(command: String) -> String {
        let chars: Vec<char> = command.chars().collect();
        let mut results = vec![];
        for (i, c) in chars.iter().enumerate() {
            match *c {
                ')' => {
                    results.push(match chars[i - 1] {
                        '(' => "o",
                        'l' => "al",
                        _ => panic!("no such chars"),
                    });
                    continue;
                }
                'G' => {
                    results.push("G");
                }
                _ => {
                    continue;
                }
            }
        }
        results.join("")
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::interpret("G()(al)".to_string()));
    println!("{}", Solution::interpret("G()()()()(al)".to_string()));
    println!("{}", Solution::interpret("(al)G(al)()()G".to_string()));
}
