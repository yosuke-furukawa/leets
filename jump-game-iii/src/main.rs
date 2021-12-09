use std::collections::HashSet;

impl Solution {
    fn bfs(pos: i32, set: &mut HashSet<i32>, arr: &[i32]) -> bool {
        if pos >= 0 && arr[pos as usize] == 0 {
            return true;
        }
        if set.contains(&pos) {
            return false;
        }

        set.insert(pos);
        let mut queue = vec![];
        let p1 = pos + arr[pos as usize];
        if p1 >= 0 && p1 < arr.len() as i32 {
            queue.push(p1);
        }

        let p2 = pos - arr[pos as usize];
        if p2 >= 0 && p2 < arr.len() as i32 {
            queue.push(p2);
        }

        for n in queue {
            if Self::bfs(n, set, arr) {
                return true;
            }
        }
        false
    }
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut set = HashSet::new();
        Self::bfs(start, &mut set, &arr)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5));
}
