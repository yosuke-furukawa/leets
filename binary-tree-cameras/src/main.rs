use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn helper(node: Option<&Rc<RefCell<TreeNode>>>, count: &mut i32) -> i32 {
        if let Some(n) = node {
            let v = n.borrow();
            let l = Self::helper(v.left.as_ref(), count);
            let r = Self::helper(v.right.as_ref(), count);
            if l == 0 || r == 0 {
                *count += 1;
                return 1;
            }
            // println!("l = {}, r = {}, count = {}, value = {}", l , r, count, v.val);
            return if l == 1 || r == 1 { 2 } else { 0 };
        }
        2
    }
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;
        if Self::helper(root.as_ref(), &mut count) == 0 {
            count += 1;
        }
        count 
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
    println!("{}", Solution::min_camera_cover(tree![0]));
    println!("{}", Solution::min_camera_cover(tree![0,0,null,0,0]));
    println!("{}", Solution::min_camera_cover(tree![0,0,null,0,null,0,null,null,0]));
    println!("{}", Solution::min_camera_cover(tree![0,0,null,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,null,0]
    ));
}
