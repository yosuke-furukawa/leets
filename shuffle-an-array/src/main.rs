use rand::seq::SliceRandom;

struct Solution {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    /** Resets the array to its original configuration and return it. */
    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    /** Returns a random shuffling of the array. */
    fn shuffle(&self) -> Vec<i32> {
        let mut copied = self.nums.clone();
        let mut rng = rand::thread_rng();
        copied.shuffle(&mut rng);
        copied
    }
}

fn main() {
    let obj = Solution::new(vec![1, 2, 3]);
    let ret_1: Vec<i32> = obj.reset();
    let ret_2: Vec<i32> = obj.shuffle();
    println!("{:?} {:?}", ret_1, ret_2);
}
