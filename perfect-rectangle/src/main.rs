use std::collections::HashSet;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut set = HashSet::new();
        let mut area = 0;
        for rect in rectangles.iter() {
            let p1 = (rect[0], rect[1]);
            let p2 = (rect[2], rect[1]);
            let p3 = (rect[2], rect[3]);
            let p4 = (rect[0], rect[3]);
            for p in [p1, p2, p3, p4] {
                if !set.insert(p) {
                    set.remove(&p);
                }
            }
            area += (p3.0 - p1.0) * (p3.1 - p1.1);
        }

        if set.len() != 4 {
            return false;
        }

        let (mut x1, mut y1, mut x2, mut y2) = (
            i32::max_value(),
            i32::max_value(),
            i32::min_value(),
            i32::min_value(),
        );
        for p in set.iter() {
            x1 = x1.min(p.0);
            y1 = y1.min(p.1);
            x2 = x2.max(p.0);
            y2 = y2.max(p.1);
        }

        area == (x2 - x1) * (y2 - y1)
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::is_rectangle_cover(vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![3, 2, 4, 4],
            vec![1, 3, 2, 4],
            vec![2, 3, 3, 4]
        ])
    );
}
