impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut pickups = vec![0; 1000];
        for trip in trips {
            if let Some(p) = pickups.get_mut(trip[1] as usize) {
                *p += trip[0];
            }
            if let Some(p) = pickups.get_mut(trip[2] as usize) {
                *p -= trip[0];
            }
        }
        let mut seats = 0;
        for pickup in pickups {
            seats += pickup;
            if seats > capacity {
                return false;
            }
        }
        true
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
    println!("{}", Solution::car_pooling(grid![[2, 1, 5], [3, 3, 7]], 4));
    println!("{}", Solution::car_pooling(grid![[2, 1, 5], [3, 3, 7]], 5));
}
