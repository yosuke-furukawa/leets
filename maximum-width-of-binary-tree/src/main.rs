use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut max_width = 1;
        queue.push_back((root, 0, 1));
        let mut levels = vec![];
        while let Some((node, level, width)) = queue.pop_front() {
            if let Some(n) = node {
                if level >= levels.len() {
                    levels.push(width);
                } else {
                    max_width = max_width.max(width - levels[level] + 1);
                }
                queue.push_back((n.borrow().left.clone(), level + 1, width * 2));
                queue.push_back((n.borrow().right.clone(), level + 1, width * 2 + 1));
            }
        }
        max_width
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

fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
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
        Solution::width_of_binary_tree(tree![1, 3, 2, 5, 3, null, 9])
    );
    println!(
        "{}",
        Solution::width_of_binary_tree(tree![1, 3, 2, 5, 3, 9, null])
    );
    println!("{}", Solution::width_of_binary_tree(tree![1, null, 2]));
    println!(
        "{}",
        Solution::width_of_binary_tree(tree![1, null, 2, 3, null, 4, null, 5])
    );
}
