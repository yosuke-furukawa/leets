impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut even = vec![];
        let mut odd = vec![];
        for num in nums {
            if num % 2 == 0 {
                even.push(num);
            } else {
                odd.push(num);
            }
        }
        even.append(&mut odd);
        even
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::sort_array_by_parity(vec![3, 1, 2, 4]));
}
