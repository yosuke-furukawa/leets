impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut dp = vec![vec![1; 5]];

        for k in 1..n as usize {
            let mut array = vec![0; 5];
            for (i, num) in dp[k - 1].iter().enumerate() {
                let to = match i {
                    0 => vec![1],
                    1 => vec![0, 2],
                    2 => vec![0, 1, 3, 4],
                    3 => vec![2, 4],
                    _ => vec![0],
                };
                for t in to {
                    array[t] = (array[t] + num) % 1_000_000_007;
                }
            }
            dp.push(array);
        }
        let mut result = 0;
        for num in dp[n as usize - 1].iter() {
            result = (result + num) % 1_000_000_007;
        }
        result
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::count_vowel_permutation(1));
    println!("{}", Solution::count_vowel_permutation(2));
    println!("{}", Solution::count_vowel_permutation(3));
    println!("{}", Solution::count_vowel_permutation(4));
    println!("{}", Solution::count_vowel_permutation(5));
    println!("{}", Solution::count_vowel_permutation(20_000));
}
