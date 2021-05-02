use std::collections::BinaryHeap;

impl Solution {
    fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut courses = courses;
        courses.sort_by_key(|n| n[1]);
        let mut current = 0;
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        for course in courses {
            let duration = course[0];
            let last_day = course[1];
            current += duration;
            heap.push(duration);
            // if current date is exceeded last_day
            if current > last_day {
                // remove course
                current -= heap.pop().unwrap();
            }
        }
        heap.len() as i32
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
        Solution::schedule_course(grid![[100, 200], [200, 1300], [1000, 1250], [2000, 3200]])
    );
}
