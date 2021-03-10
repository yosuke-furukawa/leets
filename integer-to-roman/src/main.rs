const THOUSANDS: [&str; 4] = ["", "M", "MM", "MMM"];
const HUNDREDS: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
const TENS: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
const ONES: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let n = num as usize;
        String::new()
            + THOUSANDS[n / 1000]
            + HUNDREDS[n % 1000 / 100]
            + TENS[n % 100 / 10]
            + ONES[n % 10]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::int_to_roman(3));
}
