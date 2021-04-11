use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut current = VecDeque::new();
        if let Some(r) = root {
            queue.push_back(r);
        }
        while !queue.is_empty() {
            current = queue.clone();
            queue.clear();
            for node in current.iter() {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                if let Some(l) = left {
                    queue.push_back(l);
                }
                if let Some(r) = right {
                    queue.push_back(r);
                }
            }
        }
        let mut sum = 0;
        for node in current.iter() {
            sum += node.borrow().val;
        }
        sum
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
    println!(
        "{}",
        Solution::deepest_leaves_sum(tree![1, 2, 3, 4, 5, null, 6, 7, null, null, null, null, 8])
    );
    println!(
        "{}",
        Solution::deepest_leaves_sum(tree![
            6, 7, 8, 2, 7, 1, 3, 9, null, 1, 4, null, null, null, 5
        ])
    );
}
