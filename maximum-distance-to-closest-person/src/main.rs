impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut prev: i32 = -1;
        let mut next: i32 = 0;
        let mut ans: i32 = 0;
        let len = seats.len() as i32;
        for (i, &seat) in seats.iter().enumerate() {
            if seat == 1 {
                prev = i as i32;
                continue;
            }

            while next < len && (seats[next as usize] == 0 || next < i as i32) {
                next += 1;
            }

            let left = if prev == -1 { len } else { i as i32 - prev };
            let right = if next == len { next } else { next - i as i32 };
            ans = ans.max(left.min(right));
        }
        ans
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1])
    );
}
