use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn find_min(node: Option<&Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(n) = node {
            if n.borrow().left.is_some() {
                Self::find_min(n.borrow().left.as_ref())
            } else {
                Some(n.borrow().val)
            }
        } else {
            None
        }
    }
    fn helper(node: Option<&Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(no) = node {
            let mut n = no.borrow_mut();
            if key < n.val {
                n.left = Self::helper(n.left.as_ref(), key);
            } else if key > n.val {
                n.right = Self::helper(n.right.as_ref(), key);
            } else {
                if n.left.is_none() {
                    return n.right.clone();
                } else if n.right.is_none() {
                    return n.left.clone();
                }
                let min_node = Self::find_min(n.right.as_ref());
                n.val = min_node.unwrap_or(0);
                n.right = Self::helper(n.right.as_ref(), n.val);
            }
            return Some(Rc::new(RefCell::new(TreeNode {
                val: n.val,
                left: n.left.clone(),
                right: n.right.clone(),
            })));
        }
        None
    }
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(root.as_ref(), key)
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
    println!(
        "{:?}",
        Solution::delete_node(tree![5, 3, 6, 2, 4, null, 7], 3)
    );
}
