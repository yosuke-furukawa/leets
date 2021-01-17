impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut nums = vec![1,1,1,1,1];
        let mut result = nums.iter().sum();
        for _ in 1 .. n {
            nums[0] = result;
            for i in 1 .. 4 {
                nums[i] = nums.get(i..5).unwrap().iter().sum();
            }
            result = nums.iter().sum();
        }
        result
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
