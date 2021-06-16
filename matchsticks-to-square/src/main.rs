impl Solution {
    fn backtrack(matches: &mut [i32], matchsticks: &[i32], match_index: usize) -> bool {
        if match_index == matchsticks.len() {
            return matches.iter().all(|m| *m == 0);
        }
        for j in 0..4 {
            if matches[j] - matchsticks[match_index] >= 0 {
                matches[j] -= matchsticks[match_index];
                if Solution::backtrack(matches, matchsticks, match_index + 1) {
                    return true;
                }
                matches[j] += matchsticks[match_index];
            }
        }
        false
    }
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum = matchsticks.iter().sum::<i32>();
        if sum % 4 != 0 {
            return false;
        }

        let avg = sum / 4;
        let mut matches = vec![avg; 4];
        Solution::backtrack(&mut matches, &matchsticks, 0)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::makesquare(vec![1, 1, 2, 2, 2]));
    println!(
        "{}",
        Solution::makesquare(vec![1, 2, 2, 2, 3, 2, 1, 1, 1, 1, 4])
    );
    println!("{}", Solution::makesquare(vec![3, 3, 3, 3, 4]));
    println!(
        "{}",
        Solution::makesquare(vec![
            5969561, 8742425, 2513572, 3352059, 9084275, 2194427, 1017540, 2324577, 6810719,
            8936380, 7868365, 2755770, 9954463, 9912280, 4713511
        ])
    );
    println!(
        "{}",
        Solution::makesquare(vec![5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3])
    );
}
