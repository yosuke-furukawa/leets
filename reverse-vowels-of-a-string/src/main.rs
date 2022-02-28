impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut vowels = vec![];
        for c in s.chars() {
            match c {
                'a' | 'i' | 'u' | 'e' | 'o' | 'A' | 'I' | 'U' | 'E' | 'O' => {
                    vowels.push(c);
                }
                _ => {}
            }
        }
        let mut res = String::new();
        for c in s.chars() {
            match c {
                'a' | 'i' | 'u' | 'e' | 'o' | 'A' | 'I' | 'U' | 'E' | 'O' => {
                    res.push(vowels.pop().unwrap());
                }
                _ => {
                    res.push(c);
                }
            }
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::reverse_vowels("hello".to_string()));
}
