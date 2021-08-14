impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let mut q = vec![];
        let mut prev = boxes[0];
        let mut cnt = 0;
        for b in boxes {
            if prev != b {
                q.push((prev, cnt));
                prev = b;
                cnt = 0;
            }
            cnt += 1;
        }
        q.push((prev, cnt));

        let mut dp = [[[0; 101]; 101]; 101];
        fn dc(
            l: usize,
            r: usize,
            k: usize,
            dp: &mut [[[i32; 101]; 101]; 101],
            q: &[(i32, i32)],
        ) -> i32 {
            let mut ans = dp[l][r][k];
            if ans != 0 || l >= r {
                return ans;
            }
            let tmp = k as i32 + q[l].1;
            ans = tmp * tmp + dc(l + 1, r, 0, dp, q);
            for i in l + 1..r {
                if q[i].0 != q[l].0 {
                    continue;
                }
                ans = std::cmp::max(
                    ans,
                    dc(l + 1, i, 0, dp, q) + dc(i, r, k + q[l].1 as usize, dp, q),
                );
            }
            dp[l][r][k] = ans;
            ans
        }

        dc(0, q.len(), 0, &mut dp, q.as_slice())
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::remove_boxes(vec![1, 3, 2, 2, 2, 3, 4, 3, 1])
    );
    println!("{}", Solution::remove_boxes(vec![1, 1, 1]));
    println!("{}", Solution::remove_boxes(vec![1]));
}
