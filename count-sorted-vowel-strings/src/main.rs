impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        (n+4)*(n+3)*(n+2)*(n+1)/24
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::count_vowel_strings(1));
    println!("{}", Solution::count_vowel_strings(2));
    println!("{}", Solution::count_vowel_strings(3));
    println!("{}", Solution::count_vowel_strings(4));
    println!("{}", Solution::count_vowel_strings(5));
}
