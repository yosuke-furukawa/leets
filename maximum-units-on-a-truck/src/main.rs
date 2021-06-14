impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut boxes = box_types;
        boxes.sort_unstable_by(|a, b| b[1].cmp(&a[1]));

        let mut ans = 0;
        let mut truck = 0;
        for b in boxes {
            if truck + b[0] > truck_size {
                ans += (truck_size - truck) * b[1];
                break;
            }
            truck += b[0];
            ans += b[0] * b[1];
        }
        ans
    }
}

struct Solution;

macro_rules! grid {
    ( $([$( $x:expr ),*]),* ) => {
        {
            vec![
                $(
                    vec![$($x), *],
                )*
            ]
        }
    };
}

fn main() {
    println!(
        "{}",
        Solution::maximum_units(grid![[1, 3], [2, 2], [3, 1]], 4)
    );
    println!(
        "{}",
        Solution::maximum_units(grid![[5, 10], [2, 5], [4, 7], [3, 9]], 10)
    );
}
