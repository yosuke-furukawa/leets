impl Solution {
    pub fn max_boxes_in_warehouse(boxes: Vec<i32>, warehouse: Vec<i32>) -> i32 {
        let mut boxes = boxes;
        let mut heights = warehouse;
        boxes.sort_unstable();
        for h in 1..heights.len() {
            heights[h] = heights[h].min(heights[h-1]);
        }
        heights.reverse();

        let mut count = 0;
        let mut b = 0;
        for height in heights {
            if b >= boxes.len() {
                break;
            }
            if boxes[b] <= height {
                b += 1;
                count += 1;
            }
        }
        count
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_boxes_in_warehouse(vec![4,3,4,1], vec![5,3,3,4,1]));
    println!("{}", Solution::max_boxes_in_warehouse(vec![1,2,2,3,4], vec![3,4,1,2]));
    println!("{}", Solution::max_boxes_in_warehouse(vec![1,2,3], vec![1,2,3,4]));
    println!("{}", Solution::max_boxes_in_warehouse(vec![2,3], vec![6,5,5,4,4,1,1,1,1,1,1,1,1,1,1,1]));
}
