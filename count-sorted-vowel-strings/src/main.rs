impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut mem = [1; 5];

        for _ in 1..n {
            mem[1] += mem[0];
            mem[2] += mem[1];
            mem[3] += mem[2];
            mem[4] += mem[3];
        }

        mem.iter().sum()
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
