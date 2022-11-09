#[derive(Default, Debug)]
struct StockSpanner {
    stack: Vec<i32>,
    prices: Vec<i32>,
}

impl StockSpanner {
    fn new() -> Self {
        Default::default()
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        while let Some(&p) = self.prices.last() {
            if p <= price {
                self.prices.pop();
                span += self.stack.pop().unwrap();
            } else {
                break;
            }
        }
        self.prices.push(price);
        self.stack.push(span);
        span
    }
}

fn main() {
    let mut s = StockSpanner::new();
    println!("{}", s.next(100));
    println!("{}", s.next(80));
    println!("{}", s.next(60));
    println!("{}", s.next(70));
    println!("{}", s.next(60));
    println!("{}", s.next(75));
    println!("{}", s.next(85));
}
