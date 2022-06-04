impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let sum = arr.iter().sum::<i32>();
        let avg = sum / 3;
        if avg * 3 != sum {
            return false;
        }
        let mut i: i32 = 0;
        let mut j: i32 = arr.len() as i32 - 1;
        let mut part_1 = 0;
        while i < arr.len() as i32 {
            part_1 += arr[i as usize];
            if part_1 == avg {
                break;
            }
            i += 1;
        }
        let mut part_2 = 0;
        while j >= 0 {
            part_2 += arr[j as usize];
            if part_2 == avg {
                break;
            }
            j -= 1;
        }
        i + 1 < j
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1])
    );
    println!(
        "{}",
        Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1])
    );
    println!(
        "{}",
        Solution::can_three_parts_equal_sum(vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4])
    );
}
