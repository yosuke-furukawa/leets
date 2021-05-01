impl Solution {
    fn find(parents: &Vec<i32>, x: i32) -> i32 {
        if parents[x as usize] == -1 {
            return x;
        }
        Self::find(parents, parents[x as usize])
    }
    fn union(parents: &mut Vec<i32>, x: i32, y: i32) {
        let xi = Self::find(parents, x);
        let yi = Self::find(parents, y);
        if xi != yi {
            parents[xi as usize] = yi;
        }
    }
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut parents = vec![-1; n as usize];
        for edge in edges {
            Self::union(&mut parents, edge[1], edge[0]);
        }
        let mut count = 0;
        for p in parents {
            if p == -1 {
                count += 1;
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
    println!("{}", Solution::count_components(5, grid![[0,1],[1,2],[3,4]]));
    println!("{}", Solution::count_components(4, grid![[0,1],[0,2],[1,2]]));
}
