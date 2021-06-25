impl Solution {
    fn find(parents: &[i32], x: i32) -> i32 {
        if parents[x as usize] == -1 {
            return x;
        }
        Self::find(parents, parents[x as usize])
    }
    fn union(parents: &mut Vec<i32>, x: i32, y: i32) -> bool {
        let xi = Self::find(parents, x);
        let yi = Self::find(parents, y);
        if xi != yi {
            parents[xi as usize] = yi;
            true
        } else {
            false
        }
    }
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut max = 0;
        for edge in edges.clone() {
            max = max.max(edge[0]);
            max = max.max(edge[1]);
        }
        let mut parents = vec![-1; max as usize + 1];
        for edge in edges.clone() {
            if !Self::union(&mut parents, edge[0], edge[1]) {
                return edge;
            }
        }

        vec![]
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
        Solution::find_redundant_connection(grid![[1, 2], [1, 3], [2, 3]])
    );
    println!(
        "{:?}",
        Solution::find_redundant_connection(grid![
            [9, 10],
            [5, 8],
            [2, 6],
            [1, 5],
            [3, 8],
            [4, 9],
            [8, 10],
            [4, 10],
            [6, 8],
            [7, 9]
        ])
    );
}
