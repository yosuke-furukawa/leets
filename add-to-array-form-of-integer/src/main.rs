impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut num = num;
        num.reverse();
        let mut carry = 0;
        let mut k = k;
        let mut i = 0;
        while i < num.len() || k > 0 || carry > 0 {
            let mut sum = carry;
            if i < num.len() {
                sum += num[i];
            }
            if k > 0 {
                sum += k % 10;
                k /= 10;
            }
            res.push(sum % 10);
            carry = sum / 10;
            i += 1;
        }
        res.reverse();
        res
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::add_to_array_form(vec![1, 2, 0, 0], 34));
}
