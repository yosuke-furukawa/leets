use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut wins = HashSet::new();
        let mut lose_count = HashMap::new();
        for m in matches.iter() {
            wins.insert(m[0]);
        }
        for m in matches.iter() {
            *lose_count.entry(m[1]).or_insert(0) += 1;
        }
        let mut results = vec![];
        let mut ls = vec![];
        for (id, &loses) in lose_count.iter() {
            if loses == 1 {
                ls.push(*id);
            }
            wins.remove(id);
        }
        ls.sort_unstable();
        let mut ws = wins.into_iter().collect::<Vec<i32>>();
        ws.sort_unstable();
        results.push(ws);
        results.push(ls);
        results
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
        Solution::find_winners(grid![
            [1, 3],
            [2, 3],
            [3, 6],
            [5, 6],
            [5, 7],
            [4, 5],
            [4, 8],
            [4, 9],
            [10, 4],
            [10, 9]
        ])
    );
}
