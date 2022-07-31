struct BitTree {
    tree: Vec<i32>,
    data: Vec<i32>,
    n: usize,
}

impl BitTree {
    fn new(n: usize) -> Self {
        let tree = vec![0; n];
        let data = vec![0; n];
        let n = n;
        BitTree { tree, data, n }
    }

    fn get(&self, i: usize) -> i32 {
        self.data[i]
    }

    fn sum(&self, i: usize) -> i32 {
        let mut res = 0;
        let down_iter = std::iter::successors(Some(i), |&i| {
            let j = i & (i + 1);
            if j > 0 {
                Some(j - 1)
            } else {
                None
            }
        });
        for j in down_iter {
            res += self.tree[j];
        }
        res
    }

    fn add(&mut self, i: usize, v: i32) {
        self.data[i] += v;
        let n = self.n;
        let up_iter = std::iter::successors(Some(i), |&i| {
            let j = i | (i + 1);
            if j < n {
                Some(j)
            } else {
                None
            }
        });
        for j in up_iter {
            self.tree[j] += v;
        }
    }
}

struct NumArray {
    bit_tree: BitTree,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut bit_tree = BitTree::new(n);
        for i in 0..n {
            bit_tree.add(i, nums[i]);
        }
        NumArray { bit_tree }
    }

    fn update(&mut self, i: i32, val: i32) {
        let i = i as usize;
        self.bit_tree.add(i as usize, val - self.bit_tree.get(i))
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let i = i as usize;
        let j = j as usize;
        if i > 0 {
            self.bit_tree.sum(j) - self.bit_tree.sum(i - 1)
        } else {
            self.bit_tree.sum(j)
        }
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
