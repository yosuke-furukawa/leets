use std::collections::VecDeque;
use std::collections::HashSet;

impl Solution {
    fn traverse(grid: Vec<Vec<bool>>, p: i32) -> (i32, i32) {
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        let mut visited: HashSet<i32> = HashSet::new();

        let mut max_point = (p, 0);
        queue.push_back(max_point);
        visited.insert(p);
        while !queue.is_empty() {
            let p = queue.pop_front().unwrap();

            if max_point.1 < p.1 {
                max_point = p;
            }
            for (i, b) in grid[p.0 as usize].iter().enumerate() {
                if *b && !visited.contains(&(i as i32)) {
                    visited.insert(i as i32);
                    queue.push_back((i as i32, p.1 + 1));
                }
            }
        }
        max_point
    }
    pub fn tree_diameter(edges: Vec<Vec<i32>>) -> i32 {
        let max = edges.iter().fold(0, |max, edge| max.max(edge.iter().fold(0, |max, item| max.max(*item))));
        let mut grid = vec![vec![false; (max + 1) as usize]; (max + 1) as usize];

        for edge in edges {
            grid[edge[0] as usize][edge[1] as usize] = true;
            grid[edge[1] as usize][edge[0] as usize] = true;
        }

        let point_a = Solution::traverse(grid.clone(), max);
        let point_b = Solution::traverse(grid.clone(), point_a.0);
        let point_c = Solution::traverse(grid.clone(), point_b.0);

        point_c.1
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
    println!("{}", Solution::tree_diameter(grid![[0,1],[0,2]]));
    println!("{}", Solution::tree_diameter(grid![[0,1],[1,2],[2,3],[1,4],[4,5]]));
}
