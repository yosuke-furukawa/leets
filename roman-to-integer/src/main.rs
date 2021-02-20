macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman = map! {'_' => 0, 'I' => 1, 'V' => 5, 'X' => 10, 'L' => 50, 'C' => 100, 'D' => 500, 'M' => 1000};
        let mut result = 0;
        for i in 0..s.len() {
            let ch = s.chars().nth(i).unwrap();
            let next_ch = s.chars().nth(i + 1).unwrap_or('_');
            let num = *roman.get(&ch).unwrap();
            let num_next = *roman.get(&next_ch).unwrap();

            result += match (num, num_next) {
                (_, _) if num < num_next => -num,
                _ => num,
            }
        }
        result
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::roman_to_int("III".to_string()));
    println!("{}", Solution::roman_to_int("MCMXCIV".to_string()));
    println!("{}", Solution::roman_to_int("LVIII".to_string()));
}
