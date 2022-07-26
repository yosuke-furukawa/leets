impl Solution {
    fn dp(chars: &Vec<char>, i: usize, alphas: &mut Vec<i32>) -> i32 {
        if i == 0 {
            alphas[chars[i] as usize - 'a' as usize] = 1;
            return 1;
        }
        let mut max = Self::dp(chars, i - 1, alphas);
        let pointer = chars[i - 1] as i32 - 'a' as i32;
        let current = chars[i] as i32 - 'a' as i32;
        if current - pointer == 1 || current - pointer == -25 {
            max += 1;
        } else {
            max = 1;
        }
        alphas[current as usize] = alphas[current as usize].max(max);
        max
    }
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let mut alphas = vec![0; 26];
        Self::dp(&p.chars().collect(), p.len() - 1, &mut alphas);
        alphas.iter().sum::<i32>()
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::find_substring_in_wrapround_string("a".to_string())
    );
    println!(
        "{}",
        Solution::find_substring_in_wrapround_string("cac".to_string())
    );
    println!(
        "{}",
        Solution::find_substring_in_wrapround_string("zab".to_string())
    );
}
