use std::collections::HashSet;

impl Solution {
    fn orientation(p: &[i32], q: &[i32], r: &[i32]) -> i32 {
        (q[1] - p[1]) * (r[0] - q[0]) - (q[0] - p[0]) * (r[1] - q[1])
    }
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut trees = trees;
        trees.sort_by(|a, b| a[1].cmp(&b[1]).then(a[0].cmp(&b[0])));
        let mut res = Vec::<Vec<i32>>::new();

        let n = trees.len();
        for tree in trees.iter().take(n) {
            while res.len() >= 2
                && Self::orientation(res.get(res.len() - 2).unwrap(), res.last().unwrap(), &tree)
                    > 0
            {
                res.pop();
            }
            res.push(tree.clone());
        }
        for i in (0..n).rev() {
            while res.len() >= 2
                && Self::orientation(
                    res.get(res.len() - 2).unwrap(),
                    res.last().unwrap(),
                    &trees[i],
                ) > 0
            {
                res.pop();
            }
            res.push(trees[i].clone());
        }
        let mut set = HashSet::<Vec<i32>>::new();
        for v in res {
            set.insert(v);
        }
        let mut ans = vec![];
        for v in set {
            ans.push(v);
        }
        ans
    }
}

struct Solution;

macro_rules! grid {
    ( $([$( $x:expr ),*]),* ) => {
        {
            vec![
                $(
                    vec![$($x), *],
                )*
            ]
        }
    };
}

fn main() {
    println!(
        "{:?}",
        Solution::outer_trees(grid![[1, 1], [2, 2], [2, 0], [2, 4], [3, 3], [4, 2]])
    );
}
