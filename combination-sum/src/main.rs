impl Solution {
    fn backtrack(
        candidates: &[i32],
        currents: &mut Vec<i32>,
        sum: i32,
        start: usize,
        target: i32,
        results: &mut Vec<Vec<i32>>,
    ) {
        if sum == target {
            results.push(currents.clone());
            return;
        }
        for i in start..candidates.len() {
            if sum + candidates[i] > target {
                break;
            }
            currents.push(candidates[i]);
            Solution::backtrack(
                candidates,
                currents,
                sum + candidates[i],
                i,
                target,
                results,
            );
            currents.pop();
        }
    }
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results = vec![];
        let mut currents = vec![];
        let mut candidates = candidates;
        candidates.sort_unstable();
        Solution::backtrack(&candidates, &mut currents, 0, 0, target, &mut results);
        results
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::combination_sum(vec![2, 3, 6, 7], 7));
    println!("{:?}", Solution::combination_sum(vec![2, 7, 6, 3, 5, 1], 9));
}
