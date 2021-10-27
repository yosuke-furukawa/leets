impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut arr = vec![0; 3];
        for n in nums.iter() {
            arr[*n as usize] += 1;
        }
        let mut index = 0;
        for (i, a) in arr.iter().enumerate() {
            let mut c = *a;
            while c > 0 {
                nums[index] = i as i32;
                index += 1;
                c -= 1;
            }
        }
    }
}

struct Solution;

fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut nums);
    println!("{:?}", nums);
    nums = vec![0];
    Solution::sort_colors(&mut nums);
    println!("{:?}", nums);
}
