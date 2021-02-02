use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
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

#[macro_export]
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

impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root {
            let mut node = r.as_ref().borrow_mut();
            if node.val < low {
                return Solution::trim_bst(node.right.clone(), low, high);
            } else if node.val > high {
                return Solution::trim_bst(node.left.clone(), low, high);
            }
            node.left = Solution::trim_bst(node.left.clone(), low, high);
            node.right = Solution::trim_bst(node.right.clone(), low, high);
            return Some(r.clone());
        }
        None
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::trim_bst(tree![1, 0, 2], 1, 2));
    println!(
        "{:?}",
        Solution::trim_bst(tree![3, 0, 4, null, 2, null, null, 1], 1, 3)
    );
    println!("{:?}", Solution::trim_bst(tree![1], 1, 2));
    println!("{:?}", Solution::trim_bst(tree![1, null, 2], 1, 3));
    println!("{:?}", Solution::trim_bst(tree![1, null, 2], 2, 4));
}
