impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let total_pts = card_points.iter().sum::<i32>();
        let n = card_points.len() as i32;
        let mut sum = card_points.iter().take((n - k) as usize).sum::<i32>();
        let mut max = total_pts - sum;
        for i in 0..n {
            if i + n - k >= n {
                break;
            }
            let delete = card_points[i as usize];
            let add = card_points[(i + n - k) as usize];
            sum = sum - delete + add;
            max = max.max(total_pts - sum);
        }
        max
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3));
    println!("{}", Solution::max_score(vec![2, 2, 2], 2));
    println!("{}", Solution::max_score(vec![9, 7, 7, 9, 7, 7, 9], 7));
    println!("{}", Solution::max_score(vec![1, 1000, 1], 1));
    println!(
        "{}",
        Solution::max_score(vec![1, 79, 80, 1, 1, 1, 200, 1], 3)
    );
    println!("{}", Solution::max_score(vec![100, 40, 17, 9, 73, 75], 3));
    println!(
        "{}",
        Solution::max_score(vec![11, 49, 100, 20, 86, 29, 72], 4)
    );
    println!(
        "{}",
        Solution::max_score(vec![96, 90, 41, 82, 39, 74, 64, 50, 30], 8)
    );
}
