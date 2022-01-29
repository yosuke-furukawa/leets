impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut max_area = 0;
        for i in 0..heights.len() {
            while !stack.is_empty() && heights[stack[stack.len() - 1]] >= heights[i] {
                let h = heights[stack.pop().unwrap()];
                let w = if stack.is_empty() {
                    i
                } else {
                    i - stack[stack.len() - 1] - 1
                };
                max_area = max_area.max(h * w as i32);
            }
            stack.push(i);
        }
        while !stack.is_empty() {
            let h = heights[stack.pop().unwrap()];
            let w = if stack.is_empty() {
                heights.len()
            } else {
                heights.len() - stack[stack.len() - 1] - 1
            };
            max_area = max_area.max(h * w as i32);
        }
        max_area
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3])
    );
}
