use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::rc::Rc;

impl Solution {
    fn helper(node: Option<&Rc<RefCell<TreeNode>>>, arrays: &mut Vec<BinaryHeap<i32>>, level: i32) {
        if let Some(node) = node {
            let n = node.borrow();
            if arrays.len() == level as usize {
                arrays.push(BinaryHeap::new());
            }
            arrays[level as usize].push(n.val);
            Self::helper(n.left.as_ref(), arrays, level + 1);
            Self::helper(n.right.as_ref(), arrays, level + 1);
        }
    }
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut arrays = vec![];
        Self::helper(root.as_ref(), &mut arrays, 0);
        let mut res = vec![];
        for array in arrays {
            res.push(*array.peek().unwrap());
        }
        res
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
    println!(
        "{:?}",
        Solution::largest_values(tree![1, 3, 2, 5, 3, null, 9])
    );
}
