impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        vec![celsius + 273.15, celsius * 9.0 / 5.0 + 32.0]
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::convert_temperature(36.5));
}
