impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        target_capacity == 0
            || (jug1_capacity + jug2_capacity >= target_capacity
                && target_capacity % Self::gcd(jug1_capacity, jug2_capacity) == 0)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::can_measure_water(3, 5, 4));
}
