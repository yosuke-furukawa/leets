impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let a_area = (ax2 - ax1) * (ay2 - ay1);
        let b_area = (bx2 - bx1) * (by2 - by1);
        if ax1 > bx2 || ax2 < bx1 || ay1 > by2 || ay2 < by1 {
            return a_area + b_area;
        }
        let x1 = ax1.max(bx1);
        let x2 = ax2.min(bx2);
        let y1 = ay1.max(by1);
        let y2 = ay2.min(by2);
        let area = (x2 - x1) * (y2 - y1);
        a_area + b_area - area
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2));
    println!("{}", Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2));
    println!("{}", Solution::compute_area(-2, -2, 2, 2, 3, 3, 4, 4));
}
