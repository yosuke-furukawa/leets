impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }
        let mut tokens = tokens;
        tokens.sort_unstable();
        let mut score = 0;
        let mut max_score = 0;
        let mut i = 0;
        let mut j = tokens.len() - 1;
        let mut power = power;
        while i <= j {
            if power >= tokens[i] {
                power -= tokens[i];
                score += 1;
                max_score = max_score.max(score);
                i += 1;
            } else if score > 0 {
                power += tokens[j];
                score -= 1;
                j -= 1;
            } else {
                break;
            }
        }
        max_score
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::bag_of_tokens_score(vec![100], 50));
    println!("{}", Solution::bag_of_tokens_score(vec![100, 200], 150));
    println!(
        "{}",
        Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200)
    );
    println!("{}", Solution::bag_of_tokens_score(vec![], 85));
}
