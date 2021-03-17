use rand::Rng;
use std::f64::consts::PI;

struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            x_center,
            y_center,
        }
    }

    fn rand_point(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let r = self.radius * rng.gen::<f64>().sqrt();
        let theta: f64 = rng.gen::<f64>() * 2.0 * PI;
        vec![
            self.x_center + r * theta.cos(),
            self.y_center + r * theta.sin(),
        ]
    }
}

fn main() {
    let sol = Solution::new(1.0, 0.0, 0.0);
    println!("{:?}", sol.rand_point());
}
