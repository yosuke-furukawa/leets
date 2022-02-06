impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 1;
        while i < nums.len() - 1 {
            let n1 = nums[i - 1];
            let n2 = nums[i];
            let n3 = nums[i + 1];
            if n1 == n2 && n2 == n3 {
                nums.remove(i);
                i -= 1;
            }
            i += 1;
        }
        nums.len() as i32
    }
}

struct Solution;

fn main() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    println!("{}, {:?}", Solution::remove_duplicates(&mut nums), nums);
}
