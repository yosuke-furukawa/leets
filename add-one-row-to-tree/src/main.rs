use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn helper(node: &mut Option<&Rc<RefCell<TreeNode>>>, level: i32, value: i32, depth: i32) {
        if let Some(n) = node {
            if level == depth - 1 {
                let v1 = Rc::new(RefCell::new(TreeNode::new(value)));
                let v2 = Rc::new(RefCell::new(TreeNode::new(value)));
                let mut m = n.borrow_mut();
                v1.borrow_mut().left = m.left.clone();
                v2.borrow_mut().right = m.right.clone();
                m.left = Some(v1);
                m.right = Some(v2);
            } else {
                Solution::helper(&mut n.borrow().right.as_ref(), level + 1, value, depth);
                Solution::helper(&mut n.borrow().left.as_ref(), level + 1, value, depth);
            }
        }
    }
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        v: i32,
        d: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if d == 1 {
            let head = Rc::new(RefCell::new(TreeNode::new(v)));
            head.borrow_mut().left = root;
            return Some(head);
        }
        Self::helper(&mut root.as_ref(), 1, v, d);
        root
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

struct Solution;

fn main() {
    println!("{:?}", Solution::add_one_row(tree![4, 2, 6, 3, 1, 5], 1, 2));
    println!("{:?}", Solution::add_one_row(tree![4, 2, 6, 3, 1, 5], 1, 1));
}
