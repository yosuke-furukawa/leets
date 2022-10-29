impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let pt_len = plant_time.len();
        let mut blooms = vec![0; 100001];
        for i in 0..pt_len {
            blooms[grow_time[i] as usize] += plant_time[i];
        }

        let mut current = 0;
        let mut res = 0;
        for i in (1..=100000).rev() {
            if blooms[i] == 0 {
                continue;
            }
            current += blooms[i] as usize;
            res = res.max(current + i);
        }
        res as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::earliest_full_bloom(vec![1,4,3], vec![2,3,1]));
}
