impl Solution {
    fn msb(n: i32) -> i32 {
        let mut count = -1;
        let mut n = n;
        while n > 0 {
            n >>= 1;
            count += 1;
        }
        count
    }
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut result = 0;
        let mut left = left;
        let mut right = right;
        while left > 0 && right > 0 {
            let left_count = Self::msb(left);
            let right_count = Self::msb(right);
            if left_count != right_count {
                break;
            }
            let mul = 1 << left_count;
            result += mul;
            left -= mul;
            right -= mul;
        }
        result
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::range_bitwise_and(5, 7));
}
