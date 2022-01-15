use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, HashSet<usize>> = HashMap::new();
        for (i, n) in arr.iter().enumerate() {
            map.entry(*n).or_insert_with(HashSet::new).insert(i);
        }
        let mut visited = vec![false; arr.len()];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((0, 0));
        while let Some((i, t)) = queue.pop_front() {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            if i + 1 == arr.len() {
                return t as i32;
            }
            queue.push_back((i + 1, t + 1));
            if i > 0 {
                queue.push_back((i - 1, t + 1));
            }
            if map.contains_key(&arr[i]) {
                for n in map.get(&arr[i]).unwrap() {
                    queue.push_back((*n, t + 1));
                }
                map.remove(&arr[i]);
            }
        }
        -1
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404])
    );
    println!("{}", Solution::min_jumps(vec![7]));
    println!("{}", Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]));
}
