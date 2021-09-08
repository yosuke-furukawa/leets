impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut sum = 0;
        let mut diff = vec![];
        for shift in shifts.iter().rev() {
            sum += shift % 26;
            diff.push(sum);
        }
        let mut chars: Vec<char> = s.chars().collect();

        for (i, r) in diff.iter().rev().enumerate() {
            chars[i] =
                std::char::from_u32((chars[i] as u32 - 'a' as u32 + *r as u32) % 26 + 'a' as u32)
                    .unwrap();
        }
        chars
            .iter()
            .fold(String::new(), |acc, cur| acc + &cur.to_string())
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::shifting_letters("abc".to_string(), vec![3, 5, 9])
    );
    println!(
        "{}",
        Solution::shifting_letters("aaa".to_string(), vec![1, 2, 3])
    );
    println!(
        "{}",
        Solution::shifting_letters(
            "mkgfzkkuxownxvfvxasy".to_string(),
            vec![
                505870226, 437526072, 266740649, 224336793, 532917782, 311122363, 567754492,
                595798950, 81520022, 684110326, 137742843, 275267355, 856903962, 148291585,
                919054234, 467541837, 622939912, 116899933, 983296461, 536563513
            ]
        )
    );
}
