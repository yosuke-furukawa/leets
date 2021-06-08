use std::collections::VecDeque;

impl Solution {
    pub fn shortest_distance(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> i32 {
        let mut distance = vec![vec![1_000_000_000; maze[0].len()]; maze.len()];
        let ds = [[-1, 0], [0, 1], [1, 0], [0, -1]];
        let mut queue: VecDeque<Vec<i32>> = VecDeque::new();
        distance[start[0] as usize][start[1] as usize] = 0;
        queue.push_back(start);
        while !queue.is_empty() {
            let position = queue.pop_front().unwrap();

            for d in ds.iter() {
                let p = position.clone();
                let mut x = p[0];
                let mut y = p[1];

                let mut path = -1;
                while x >= 0
                    && (x as usize) < maze.len()
                    && y >= 0
                    && (y as usize) < maze[0].len()
                    && maze[x as usize][y as usize] == 0
                {
                    x += d[0];
                    y += d[1];
                    path += 1;
                }
                let new_pos = vec![x - d[0], y - d[1]];
                if distance[p[0] as usize][p[1] as usize] + path
                    < distance[(x - d[0]) as usize][(y - d[1]) as usize]
                {
                    distance[(x - d[0]) as usize][(y - d[1]) as usize] =
                        distance[p[0] as usize][p[1] as usize] + path;
                    queue.push_back(new_pos);
                }
            }
        }
        if distance[destination[0] as usize][destination[1] as usize] == 1_000_000_000 {
            -1
        } else {
            distance[destination[0] as usize][destination[1] as usize]
        }
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
        "{}",
        Solution::shortest_distance(
            grid![
                [0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 1, 0],
                [1, 1, 0, 1, 1],
                [0, 0, 0, 0, 0]
            ],
            vec![0, 4],
            vec![4, 4]
        )
    );
    println!(
        "{}",
        Solution::shortest_distance(
            grid![
                [0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 1, 0],
                [1, 1, 0, 1, 1],
                [0, 0, 0, 0, 0]
            ],
            vec![0, 4],
            vec![3, 2]
        )
    );
    println!(
        "{}",
        Solution::shortest_distance(
            grid![
                [0, 0, 0, 0, 0],
                [1, 1, 0, 0, 1],
                [0, 0, 0, 0, 0],
                [0, 1, 0, 0, 1],
                [0, 1, 0, 0, 0]
            ],
            vec![4, 3],
            vec![0, 1]
        )
    );
}
