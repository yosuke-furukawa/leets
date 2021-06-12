use std::collections::BinaryHeap;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut stations = stations;
        stations.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut current = start_fuel;
        let mut count = 0;
        let mut station_index = 0;
        while current < target {
            count += 1;
            while station_index < stations.len() && stations[station_index][0] <= current {
                heap.push(stations[station_index][1]);
                station_index += 1;
            }
            if heap.is_empty() {
                break;
            }
            current += heap.pop().unwrap();
        }
        if current >= target {
            count
        } else {
            -1
        }
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
    println!("{}", Solution::min_refuel_stops(1, 1, grid![]));
    println!("{}", Solution::min_refuel_stops(100, 1, grid![[10, 100]]));
    println!(
        "{}",
        Solution::min_refuel_stops(100, 10, grid![[10, 60], [20, 30], [30, 30], [60, 40]])
    );
}
