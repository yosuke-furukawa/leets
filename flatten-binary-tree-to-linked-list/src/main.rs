use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }

        let mut curr = Rc::clone(root.as_ref().unwrap());
        while curr.borrow().right.is_some() || curr.borrow().left.is_some() {
            let right = curr.borrow_mut().right.take();
            let left = curr.borrow_mut().left.take();
            curr.borrow_mut().right = left;
            let mut rmax = Rc::clone(&curr);
            while rmax.borrow().right.is_some() {
                let x = Rc::clone(rmax.borrow().right.as_ref().unwrap());
                rmax = x;
            }
            rmax.borrow_mut().right = right;
            let next = Rc::clone(curr.borrow().right.as_ref().unwrap());
            curr = next;
        }
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
    let mut root = tree![1, 2, 5, 3, 4, null, 6];
    Solution::flatten(&mut root);
    println!("{:?}", root);
}
