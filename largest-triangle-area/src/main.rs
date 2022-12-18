impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut max = 0.0;
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                for k in j + 1..points.len() {
                    let area = 0.5
                        * ((points[i][0] * points[j][1]
                            + points[j][0] * points[k][1]
                            + points[k][0] * points[i][1])
                            - (points[i][1] * points[j][0]
                                + points[j][1] * points[k][0]
                                + points[k][1] * points[i][0]))
                            .abs() as f64;
                    if area > max {
                        max = area;
                    }
                }
            }
        }
        max
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
        "{}",
        Solution::largest_triangle_area(grid![[0, 0], [0, 1], [1, 0], [0, 2], [2, 0]])
    );
}
