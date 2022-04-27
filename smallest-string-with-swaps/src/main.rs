use std::cmp::Reverse;
use std::collections::HashMap;

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        UnionFind { parent }
    }

    fn find(&mut self, i: usize) -> usize {
        let j = self.parent[i];
        if i == j {
            i
        } else {
            let k = self.find(j);
            self.parent[i] = k;
            k
        }
    }

    fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);
        if i != j {
            self.parent[i] = j;
        }
    }
}

impl Solution {
    fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let n = s.len();
        let mut uf = UnionFind::new(n);
        let mut hm: HashMap<usize, Vec<char>> = HashMap::new();
        for pair in pairs {
            uf.union(pair[0] as usize, pair[1] as usize);
        }
        for (i, c) in s.char_indices() {
            hm.entry(uf.find(i)).or_default().push(c);
        }
        for v in hm.values_mut() {
            v.sort_unstable_by_key(|&x| Reverse(x));
        }
        let mut res: Vec<char> = vec![];
        for i in 0..n {
            res.push(hm.entry(uf.find(i)).or_default().pop().unwrap());
        }
        res.into_iter().collect()
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::smallest_string_with_swaps("dcab".to_string(), vec![vec![0, 3], vec![1, 2]])
    );
}
