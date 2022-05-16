impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut result = score.iter().map(|n| n.to_string()).collect::<Vec<String>>();
        let mut ranks = score.iter().enumerate().collect::<Vec<_>>();
        ranks.sort_by(|a, b| b.1.cmp(a.1));
        for i in 0..ranks.len() {
            if i == 0 {
                result[ranks[i].0] = "Gold Medal".to_string();
            } else if i == 1 {
                result[ranks[i].0] = "Silver Medal".to_string();
            } else if i == 2 {
                result[ranks[i].0] = "Bronze Medal".to_string();
            } else {
                result[ranks[i].0] = (i + 1).to_string();
            }
        }
        result
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]));
    println!("{:?}", Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]));
}
