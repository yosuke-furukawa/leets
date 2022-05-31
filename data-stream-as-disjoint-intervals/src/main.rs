use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Default)]
struct SummaryRanges {
    intervals: VecDeque<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {
    fn new() -> Self {
        Self::default()
    }

    fn add_num(&mut self, val: i32) {
        let result = self.intervals.binary_search_by(|interval| {
            if interval[0] > val {
                Ordering::Greater
            } else if interval[1] < val {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        if result.is_ok() {
            return;
        }
        let index = result.unwrap_err();
        if index >= self.intervals.len() {
            self.intervals.push_back(vec![val, val]);
        } else if index == 0 {
            self.intervals.push_front(vec![val, val]);
        } else {
            self.intervals.insert(index, vec![val, val]);
        }
        let mut i = 0;
        while i < self.intervals.len() {
            if i + 1 < self.intervals.len() && self.intervals[i][1] + 1 >= self.intervals[i + 1][0]
            {
                self.intervals[i][1] = self.intervals[i + 1][1];
                self.intervals.remove(i + 1);
                continue;
            }
            i += 1;
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.intervals.clone().into_iter().collect()
    }
}

fn main() {
    let mut summary_range = SummaryRanges::new();
    summary_range.add_num(1);
    println!("{:?}", summary_range.get_intervals());
    summary_range.add_num(3);
    println!("{:?}", summary_range.get_intervals());
    summary_range.add_num(7);
    println!("{:?}", summary_range.get_intervals());
    summary_range.add_num(2);
    println!("{:?}", summary_range.get_intervals());
    summary_range.add_num(6);
    println!("{:?}", summary_range.get_intervals());
}
