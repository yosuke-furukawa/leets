use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
        let init:HashSet<i32> = mat.clone().into_iter().flatten().collect();
        let intersection = mat.into_iter().map(HashSet::from_iter).try_fold(init, |set, temp| {
            match (set, temp) {
                (set, _) if set.is_empty() => None,
                (set, temp) => Some(set.intersection(&temp).copied().collect()),
            }
        });
        if intersection.is_none() {
            return -1;
        }
        let mut arr: Vec<i32> = intersection.unwrap().iter().copied().collect();
        arr.sort_unstable();
        *arr.first().unwrap()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::smallest_common_element(vec![vec![1,2,3,4,5],vec![2,4,5,8,10],vec![3,5,7,9,11],vec![1,3,5,7,9]]));
}
