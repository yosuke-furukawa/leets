use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn get_min(node: Option<&Rc<RefCell<TreeNode>>>, array: &mut Vec<i32>) {
        if let Some(n) = node {
            let b = n.borrow();
            Self::get_min(b.left.as_ref(), array);
            array.push(b.val);
            Self::get_min(b.right.as_ref(), array);
        }
    }
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut array = vec![];
        Self::get_min(root.as_ref(), &mut array);
        let mut min = std::i32::MAX;
        for i in 0..array.len() - 1 {
            min = min.min(array[i + 1] - array[i]);
        }
        min
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
    println!("{}", Solution::get_minimum_difference(tree![4, 2, 6, 1, 3]));
}
