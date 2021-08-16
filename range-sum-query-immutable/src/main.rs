use std::collections::HashMap;

struct NumArray {
    map: HashMap<usize, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut map = HashMap::new();
        let mut sum = 0;
        for (i, n) in nums.iter().enumerate() {
            sum += n;
            map.insert(i, sum);
        }
        Self { map }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left > 0 {
            self.map.get(&(right as usize)).unwrap() - self.map.get(&(left as usize - 1)).unwrap()
        } else {
            *self.map.get(&(right as usize)).unwrap()
        }
    }
}

fn main() {
    let obj = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    println!("{}", obj.sum_range(0, 2));
    println!("{}", obj.sum_range(2, 5));
    println!("{}", obj.sum_range(0, 5));
}
