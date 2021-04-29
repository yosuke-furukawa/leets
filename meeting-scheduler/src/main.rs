use std::collections::VecDeque;

impl Solution {
    pub fn min_available_duration(
        slots1: Vec<Vec<i32>>,
        slots2: Vec<Vec<i32>>,
        duration: i32,
    ) -> Vec<i32> {
        let mut slots1 = slots1;
        let mut slots2 = slots2;
        slots1.sort_unstable();
        slots2.sort_unstable();
        let mut queue1: VecDeque<Vec<i32>> = slots1.into_iter().collect();
        let mut queue2: VecDeque<Vec<i32>> = slots2.into_iter().collect();
        while !queue1.is_empty() && !queue2.is_empty() {
            let item1 = queue1.pop_front().unwrap();
            let item2 = queue2.pop_front().unwrap();
            if item1[0] <= item2[1] && item1[1] >= item2[0] {
                let start = item1[0].max(item2[0]);
                let diff = item1[1].min(item2[1]) - start;
                if diff >= duration {
                    return vec![start, start + duration];
                }
            }
            if item2[1] > item1[1] {
                queue2.push_front(item2);
            } else {
                queue1.push_front(item1);
            }
        }
        vec![]
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
        "{:?}",
        Solution::min_available_duration(
            grid![[10, 50], [60, 120], [140, 210]],
            grid![[0, 15], [60, 70]],
            8
        )
    );
    println!(
        "{:?}",
        Solution::min_available_duration(
            grid![[10, 50], [60, 120], [140, 210]],
            grid![[0, 15], [60, 70]],
            12
        )
    );
    println!(
        "{:?}",
        Solution::min_available_duration(
            grid![
                [216397070, 363167701],
                [98730764, 158208909],
                [441003187, 466254040],
                [558239978, 678368334],
                [683942980, 717766451]
            ],
            grid![
                [50490609, 222653186],
                [512711631, 670791418],
                [730229023, 802410205],
                [812553104, 891266775],
                [230032010, 399152578]
            ],
            456085
        )
    );
}
