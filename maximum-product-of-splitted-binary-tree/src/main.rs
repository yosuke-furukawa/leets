use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn sum(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(n) = node {
            let mut val = n.borrow().val;

            if n.borrow().left.is_some() {
                val += Self::sum(n.borrow().left.as_ref());
            }

            if n.borrow().right.is_some() {
                val += Self::sum(n.borrow().right.as_ref());
            }
            return val;
        }
        0
    }
    fn product(node: Option<&Rc<RefCell<TreeNode>>>, sum: i32, max: &mut u64) -> i32 {
        if let Some(n) = node {
            let left = if n.borrow().left.is_some() {
                Self::product(n.borrow().left.as_ref(), sum, max)
            } else {
                0
            };

            let right = if n.borrow().right.is_some() {
                Self::product(n.borrow().right.as_ref(), sum, max)
            } else {
                0
            };

            let val = n.borrow().val;
            let res = val + left + right;
            *max = (*max).max((sum - res) as u64 * res as u64);
            return res;
        }
        0
    }
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let sum = Solution::sum(root.as_ref());
        let mut max = 0;
        Solution::product(root.as_ref(), sum, &mut max);
        (max % 1_000_000_007) as i32
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

pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;
    let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
    let mut queue = VecDeque::new();
    queue.push_back(head.as_ref().unwrap().clone());

    for children in vec[1..].chunks(2) {
        let parent = queue.pop_front().unwrap();
        if let Some(v) = children[0] {
            parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
            queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
        }
        if children.len() > 1 {
            if let Some(v) = children[1] {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
    }
    head
}

macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            to_tree(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}

fn main() {
    println!("{}", Solution::max_product(tree![1, 2, 3, 4, 5, 6]));
}
