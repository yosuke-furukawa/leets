impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        fn dfs(
            idx: i32,
            rem: i32,
            c: i32,
            dp: &mut Vec<Vec<Vec<i32>>>,
            hs: &Vec<i32>,
            cs: &Vec<Vec<i32>>,
            m: i32,
            n: i32,
        ) -> i32 {
            if rem >= 0 && dp[idx as usize][rem as usize][c as usize] != -1 {
                return dp[idx as usize][rem as usize][c as usize];
            }
            let mut ans = std::i32::MAX;
            if idx >= m || rem < 0 {
                return if rem == 0 { 0 } else { ans };
            }
            if hs[idx as usize] > 0 {
                let diff = if hs[idx as usize] == c { 0 } else { 1 };
                dp[idx as usize][rem as usize][c as usize] =
                    dfs(idx + 1, rem - diff, hs[idx as usize], dp, hs, cs, m, n);
                return dp[idx as usize][rem as usize][c as usize];
            }
            for i in 1..=n {
                let diff = if i == c { 0 } else { 1 };
                let last = dfs(idx + 1, rem - diff, i, dp, hs, cs, m, n);
                {
                    if last < std::i32::MAX {
                        ans = std::cmp::min(
                            ans,
                            if i == 0 {
                                0
                            } else {
                                cs[idx as usize][i as usize - 1] + last
                            },
                        );
                    }
                }
            }
            dp[idx as usize][rem as usize][c as usize] = ans;
            ans
        }
        let mut dp = vec![vec![vec![-1; n as usize + 2]; target as usize + 1]; m as usize + 1];
        for i in 1..=n {
            dp[m as usize][0][i as usize] = 0;
        }
        let ans = dfs(0, target, n + 1, &mut dp, &houses, &cost, m, n);
        if ans < std::i32::MAX {
            ans
        } else {
            -1
        }
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_cost(vec![0,0,0,0,0], vec![vec![1,10],vec![10,1],vec![10,1],vec![1,10],vec![5,1]], 5, 2, 3));
}
