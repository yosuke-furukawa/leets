use std::collections::VecDeque;

impl Solution {
    pub fn can_visit_all_rooms(mut rooms: Vec<Vec<i32>>) -> bool {
        let mut queue: VecDeque<i32> = VecDeque::new();
        for k in rooms[0].iter() {
            queue.push_back(*k);
        }
        rooms[0] = vec![];

        while !queue.is_empty() {
            if let Some(key) = queue.pop_front() {
                if key < rooms.len() as i32 && !rooms[key as usize].is_empty() {
                    for k in rooms[key as usize].iter() {
                        queue.push_back(*k);
                    }
                    rooms[key as usize] = vec![];
                }
            }
        }
        rooms.iter().all(|a| a.is_empty())
    }
}

struct Solution;

#[macro_export]
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
        "{}",
        Solution::can_visit_all_rooms(grid![[1], [2], [3], []])
    );
    println!(
        "{}",
        Solution::can_visit_all_rooms(grid![[1, 3], [3, 0, 1], [2], [0]])
    );
}
