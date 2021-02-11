impl Solution {
    pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![0; length as usize];

        for update in updates {
            let s = update[0];
            let e = update[1];
            let n = update[2];
            result[s as usize] += n;

            if e + 1 < length {
                result[(e + 1) as usize] -= n;
            }
        }

        let mut sum = 0;
        for r in result.iter_mut() {
            sum += *r;
            *r = sum;
        }

        result
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::get_modified_array(5, vec![vec![1, 3, 2], vec![2, 4, 3], vec![0, 2, -2]])
    );
}
