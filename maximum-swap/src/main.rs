impl Solution {
    fn check_max(digits: &[i32]) -> Option<(i32, usize)> {
        if digits.is_empty() {
            return None;
        }
        let mut max = *digits.last().unwrap();
        let mut max_index = None;
        for (index, digit) in digits.iter().enumerate() {
            if *digit > max {
                max = *digit;
                max_index = Some(index);
            }
        }
        max_index.map(|max_index| (max, max_index))
    }
    pub fn maximum_swap(num: i32) -> i32 {
        let mut digits = Vec::new();
        let mut n = num;
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        let mut result = 0;
        while Self::check_max(&digits).is_none() && !digits.is_empty() {
            result = result * 10 + digits.pop().unwrap();
        }
        if let Some((_, max_index)) = Self::check_max(&digits) {
            let len = digits.len();
            digits.swap(len - 1, max_index);
        }
        while let Some(n) = digits.pop() {
            result = result * 10 + n;
        }
        result
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::maximum_swap(2736));
    println!("{}", Solution::maximum_swap(12));
    println!("{}", Solution::maximum_swap(98368));
    println!("{}", Solution::maximum_swap(1993));
}
