use std::collections::BinaryHeap;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let (n1, n2) = (nums1.len(), nums2.len());
        let mut heap: BinaryHeap<(i32, usize, usize)> = BinaryHeap::from(
            (0..n1)
                .map(|x| (-nums1[x] - nums2[0], x, 0))
                .collect::<Vec<(i32, usize, usize)>>(),
        );

        while let Some((_, i, j)) = heap.pop() {
            res.push(vec![nums1[i], nums2[j]]);
            if res.len() as i32 == k {
                break;
            }
            if j < n2 - 1 {
                heap.push((-nums1[i] - nums2[j + 1], i, j + 1));
            }
        }

        res
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3)
    );
}
