impl Solution {
    fn helper(
        sub: &[i32],
        candidates: &[i32],
        skipped: usize,
        target: &mut i32,
        results: &mut Vec<Vec<i32>>,
    ) {
        if *target == 0 {
            results.push(sub.to_vec());
            return;
        }

        if *target < 0 {
            return;
        }

        for (i, v) in candidates.iter().enumerate().skip(skipped) {
            if i > skipped && candidates[i] == candidates[i - 1] {
                continue;
            }
            let mut s = sub.to_vec();
            s.push(*v);
            Self::helper(&s, candidates, i + 1, &mut (*target - *v), results);
        }
    }
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        let mut target = target;
        candidates.sort_unstable();
        let mut results: Vec<Vec<i32>> = vec![];
        Self::helper(&[], &candidates, 0, &mut target, &mut results);
        results
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)
    );
}
