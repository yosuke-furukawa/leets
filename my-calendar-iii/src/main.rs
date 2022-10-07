use std::collections::BTreeMap;

#[derive(Default)]
struct MyCalendarThree {
    map: BTreeMap<i32, i32>,
}

impl MyCalendarThree {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        *self.map.entry(start).or_default() += 1;
        *self.map.entry(end).or_default() -= 1;
        let mut res = 0;
        let mut count = 0;
        for (_, v) in self.map.iter() {
            count += v;
            res = res.max(count);
        }
        res
    }
}

fn main() {
    let mut obj = MyCalendarThree::new();
    assert_eq!(1, obj.book(10, 20));
    assert_eq!(1, obj.book(50, 60));
    assert_eq!(2, obj.book(10, 40));
    assert_eq!(3, obj.book(5, 15));
    assert_eq!(3, obj.book(5, 10));
    assert_eq!(3, obj.book(25, 55));
}
