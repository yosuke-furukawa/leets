use std::collections::HashSet;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut candidates = HashSet::new();
        for building in buildings.iter() {
            candidates.insert(building[0]);
            candidates.insert(building[1]);
        }
        let mut candidates = candidates.into_iter().collect::<Vec<i32>>();
        candidates.sort_unstable();
        let mut result = vec![vec![-1, 0]];
        for candidate in candidates {
            let mut i = 0;
            let mut height = 0;
            while i < buildings.len() && buildings[i][0] <= candidate {
                if buildings[i][1] > candidate {
                    height = height.max(buildings[i][2]);
                }
                i += 1;
            }
            if result.last().unwrap()[1] != height {
                result.push(vec![candidate, height]);
            }
        }
        result.remove(0);
        result
    }
}

struct Solution;

macro_rules! grid {
    ($($x:expr),*) => (vec![$($x.to_vec()),*]);
}

fn main() {
    println!(
        "{:?}",
        Solution::get_skyline(grid![
            [2, 9, 10],
            [3, 7, 15],
            [5, 12, 12],
            [15, 20, 10],
            [19, 24, 8]
        ])
    );
}
