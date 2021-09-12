use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, m: i32, _n: i32) -> i32 {
        let mut e = HashMap::<i32, HashMap<i32, i32>>::new();
        for v in &edges {
            e.entry(v[0]).or_default().insert(v[1], v[2]);
            e.entry(v[1]).or_default().insert(v[0], v[2]);
        }
        let mut heap = BinaryHeap::<(i32, i32)>::new();
        heap.push((m, 0));
        let mut visited = HashMap::<i32, i32>::new();
        while !heap.is_empty() {
            let x = heap.pop().unwrap();
            let moves = x.0;
            let i = x.1;
            if !visited.contains_key(&i) {
                visited.insert(i, moves);
                for j in e.get(&i).unwrap_or(&HashMap::<i32, i32>::new()).keys() {
                    let moves2 = moves - e.get(&i).unwrap().get(j).unwrap() - 1;
                    if !visited.contains_key(j) && moves2 >= 0 {
                        heap.push((moves2, *j));
                    }
                }
            }
        }
        let mut ans = visited.len() as i32;
        for v in &edges {
            let a = visited.get(&v[0]).unwrap_or(&0);
            let b = visited.get(&v[1]).unwrap_or(&0);
            ans += (a + b).min(v[2]);
        }
        ans
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::reachable_nodes(vec![vec![0, 1, 10], vec![0, 2, 1], vec![1, 2, 2]], 6, 3)
    );
}
