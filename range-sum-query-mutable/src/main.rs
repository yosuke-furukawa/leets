struct NumArray {
    array: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self { array: nums }
    }

    fn update(&mut self, index: i32, val: i32) {
        self.array[index as usize] = val;
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.array
            .iter()
            .skip(left as usize)
            .take(right as usize - left as usize + 1)
            .sum()
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */

fn main() {
    let mut obj = NumArray::new(vec![1, 3, 5]);
    println!("{}", obj.sum_range(0, 2));
    obj.update(1, 2);
    println!("{}", obj.sum_range(0, 2));
    println!("{}", obj.sum_range(2, 2));
    println!("{}", obj.sum_range(1, 1));
    let obj2 = NumArray::new(vec![0, 9, 5, 7, 3]);
    println!("{}", obj2.sum_range(4, 4));
    println!("{}", obj2.sum_range(2, 4));
    println!("{}", obj2.sum_range(3, 3));
}
