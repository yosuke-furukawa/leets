impl Solution {
    pub fn find_judge(n: i32, trusts: Vec<Vec<i32>>) -> i32 {
        let mut truster = vec![0; n as usize];
        let mut trustee = vec![0; n as usize];
        for trust in trusts {
            truster[trust[0] as usize - 1] += 1;
            trustee[trust[1] as usize - 1] += 1;
        }
        for i in 1..=n {
            match (truster[i as usize - 1], trustee[i as usize - 1]) {
                (0, k) if k == n - 1 => {
                    return i;
                }
                _ => {
                    continue;
                }
            }
        }
        -1
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
    println!("{}", Solution::find_judge(2, grid![[1, 2]]));
    println!("{}", Solution::find_judge(3, grid![[1, 3], [2, 3]]));
    println!("{}", Solution::find_judge(3, grid![[1, 3], [2, 3], [3, 1]]));
}
