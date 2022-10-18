impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n < 1 {
            return "".to_string();
        }
        if n == 1 {
            return "1".to_string();
        }
        let mut chars = vec!['1'];
        for _ in 1..n {
            let mut news = "".to_string();
            let mut count = 1;
            for (i, &c) in chars.iter().enumerate().skip(1) {
                if c == chars[i - 1] {
                    count += 1;
                } else {
                    news.push_str(&count.to_string());
                    news.push(chars[i - 1]);
                    count = 1;
                }
            }
            news.push_str(&count.to_string());
            news.push(chars[chars.len() - 1]);
            chars = news.chars().collect();
        }
        chars.into_iter().collect()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::count_and_say(4));
    println!("{}", Solution::count_and_say(5));
    println!("{}", Solution::count_and_say(6));
    println!("{}", Solution::count_and_say(7));
}
