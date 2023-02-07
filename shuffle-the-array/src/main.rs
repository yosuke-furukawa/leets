impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 0..n {
            let nth = nums[i as usize + n as usize];
            res.push(nums[i as usize]);
            res.push(nth);
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3));
}
