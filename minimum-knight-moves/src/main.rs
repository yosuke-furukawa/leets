use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn min_knight_moves(x: i32, y: i32) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back((0, 0, 0));
        let dx = &[-2, -1, 1, 2, -2, -1, 1, 2];
        let dy = &[-1, -2, -2, -1, 1, 2, 2, 1];
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        while !queue.is_empty() {
            let pos = queue.pop_front().unwrap();
            if pos.0 == x && pos.1 == y {
                return pos.2;
            }
            if visited.contains(&(pos.0, pos.1)) {
                continue;
            }
            visited.insert((pos.0, pos.1));
            for i in 0..8 {
                queue.push_back((pos.0 + dx[i], pos.1 + dy[i], pos.2 + 1));
            }
        }
        -1
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_knight_moves(2, 1));
}
