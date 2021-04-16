impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack: Vec<(char, i32)> = vec![];

        for c in s.chars() {
            if stack.is_empty() {
                stack.push((c, 1));
                continue;
            }

            let (ch, count) = stack[stack.len() - 1];
            if ch == c {
                stack.push((c, count + 1));
            } else {
                stack.push((c, 1));
            }

            if let Some(peek) = stack.last() {
                let mut removal = peek.1;
                if removal == k {
                    while removal > 0 {
                        stack.pop();
                        removal -= 1;
                    }
                }
            }
        }

        stack.iter().fold(String::new(), |mut acc, cur| {
            acc.push(cur.0);
            acc
        })
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::remove_duplicates("abcd".to_string(), 2));
    println!(
        "{}",
        Solution::remove_duplicates("deeedbbcccbdaa".to_string(), 3)
    );
    println!(
        "{}",
        Solution::remove_duplicates("pbbcggttciiippooaais".to_string(), 2)
    );
}
