impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max: i32 = 0;
        let mut start: usize = 0;
        let mut end: usize = height.len() - 1;
        while start < end {
            max = max.max(height[start].min(height[end]) * (end - start) as i32);
            if height[start] < height[end] {
                start += 1;
            } else {
                end -= 1;
            }
        }
        max
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}
