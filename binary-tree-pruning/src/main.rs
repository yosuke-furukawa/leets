use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn helper(node: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(n) = node {
            if n.borrow().left.is_none() && n.borrow().right.is_none() {
                return n.borrow().val == 0;
            }
            let left_removable = if n.borrow().left.is_some() {
                let removable = Self::helper(n.borrow().left.as_ref());
                if removable {
                    let mut node = n.borrow_mut();
                    node.left = None;
                }
                removable
            } else {
                true
            };

            let right_removable = if n.borrow().right.is_some() {
                let removable = Self::helper(n.borrow().right.as_ref());
                if removable {
                    let mut node = n.borrow_mut();
                    node.right = None;
                }
                removable
            } else {
                true
            };

            return left_removable && right_removable && n.borrow().val == 0;
        }
        false
    }
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if Self::helper(root.as_ref()) {
            return None;
        }
        root
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
    println!("{:?}", Solution::prune_tree(tree![1, null, 0, 0, 1]));
    println!("{:?}", Solution::prune_tree(tree![0, null, 0, 0, 0]));
}
