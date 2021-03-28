use std::collections::HashSet;

impl Solution {
    fn backtrack(grid: &[Vec<i32>], position: (usize, usize), visited: &mut HashSet<(usize, usize)>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let d = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        if position.0 >= m || position.1 >= n || grid[position.0][position.1] == 0 || visited.contains(&position) {
            return 0;
        }

        let mut cur = 0;
        visited.insert(position);
        for (dx, dy) in d.iter() {
            cur = cur.max(Self::backtrack(grid, ((position.0 as i32 + dx) as usize, (position.1 as i32 + dy) as usize), visited));
        }
        visited.remove(&position);
        cur + grid[position.0][position.1]
    }
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                if grid[x][y] != 0 {
                    let mut visited = HashSet::new();
                    let c = Self::backtrack(&grid, (x, y), &mut visited);
                    count = count.max(c);
                }
            }
        }
        count
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
    println!("{}", Solution::get_maximum_gold(grid![[0,6,0],[5,8,7],[0,9,0]]));
}
