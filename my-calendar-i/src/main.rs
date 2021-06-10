use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default)]
struct MyCalendar {
    calendar: BinaryHeap<Reverse<(i32, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self::default()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for Reverse((s, e)) in self.calendar.iter() {
            if start < *e && end > *s {
                return false;
            }
        }
        self.calendar.push(Reverse((start, end)));
        true
    }
}

fn main() {
    let mut cal = MyCalendar::new();
    println!("{}", cal.book(10, 20));
    println!("{}", cal.book(15, 25));
    println!("{}", cal.book(20, 30));
    println!("{}", cal.book(1, 30));
    println!("{}", cal.book(2, 30));
    println!("{}", cal.book(3, 30));
    println!("{}", cal.book(200, 30));
}
