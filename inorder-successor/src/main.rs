use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

impl Solution {
    fn helper(
        node: Option<&Rc<RefCell<TreeNode>>>,
        p: i32,
        successor: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(n) = node {
            let node = n.borrow();
            Self::helper(node.left.as_ref(), p, successor);
            if successor.is_none() && node.val > p {
                *successor = Some(Rc::new(RefCell::new(TreeNode::new(node.val))));
            }
            Self::helper(node.right.as_ref(), p, successor);
        }
    }

    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p.as_ref().unwrap().borrow().val;
        let mut found = None;
        Self::helper(root.as_ref(), p, &mut found);
        found
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
        "{:?}",
        Solution::inorder_successor(tree![2, 1, 3], tree![1])
    );
    println!(
        "{:?}",
        Solution::inorder_successor(tree![5, 3, 6, 2, 4, null, null, 1], tree![6])
    );
}
