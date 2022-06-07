impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut less = vec![];
        let mut equal = vec![];
        let mut greater = vec![];
        for num in nums {
            match num {
                x if x < pivot => less.push(x),
                x if x == pivot => equal.push(x),
                x if x > pivot => greater.push(x),
                _ => (),
            }
        }
        less.append(&mut equal);
        less.append(&mut greater);
        less
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10)
    );
}
