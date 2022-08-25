use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let mut max_index = 0;
        for (i, &num) in nums.iter().enumerate() {
            if nums[max_index] < num {
                max_index = i;
            }
        }
        let node = Rc::new(RefCell::new(TreeNode::new(nums[max_index])));
        node.borrow_mut().left =
            Solution::construct_maximum_binary_tree(nums[0..max_index].to_vec());
        node.borrow_mut().right =
            Solution::construct_maximum_binary_tree(nums[max_index + 1..].to_vec());
        Some(node)
    }
}

struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5])
    );
}
