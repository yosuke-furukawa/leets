impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envs = envelopes;
        envs.sort_unstable_by(|a, b| {
            let diff = a[0].cmp(&b[0]);
            if  diff == std::cmp::Ordering::Equal {
                return b[1].cmp(&a[1]);
            }
            diff
        });

        let second_dim: Vec<i32> = envs.iter().map(|e| e[1]).collect();
        let mut dp = vec![];
        for n in second_dim {
            let i = Self::search(&dp, n);
            if i < 0  {
                dp.push(n);
            } else {
                dp[i as usize] = n;
            }
        } 
        dp.len() as i32
    }
    fn search(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;
        let mut ans = -1;
        while left <= right {
            let mid = (right + left) / 2;

            let m = nums[mid as usize];
            if m < target {
                left = mid + 1;
            } else {
                ans = mid;
                right = mid - 1;
            }
        }
        ans
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
    //println!("{}", Solution::max_envelopes(grid![[5,4],[6,4],[6,7],[2,3]]));
    println!("{}", Solution::max_envelopes(grid![[4,5],[4,6],[6,7],[2,3],[1,1]]));
}
