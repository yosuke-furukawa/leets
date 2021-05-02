impl Solution {
    pub fn bad_sensor(sensor1: Vec<i32>, sensor2: Vec<i32>) -> i32 {
        let mut diff = -1;
        for i in 0..sensor1.len() {
            if sensor1[i] == sensor2[i] {
                continue;
            }
            if sensor1.get(i + 1..) == sensor2.get(i..sensor2.len() - 1) {
                diff = 2;
            } else if sensor1.get(i..sensor1.len() - 1) == sensor2.get(i + 1..) {
                diff = 1;
            }
            break;
        }
        diff
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::bad_sensor(vec![2, 3, 4, 5], vec![2, 1, 3, 4])
    );
    println!(
        "{}",
        Solution::bad_sensor(
            vec![8, 2, 2, 6, 3, 8, 7, 2, 5, 3],
            vec![2, 8, 2, 2, 6, 3, 8, 7, 2, 5]
        )
    );
}
