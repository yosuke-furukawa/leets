use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn helper(
        node: Option<&Rc<RefCell<TreeNode>>>,
        target: Option<&Rc<RefCell<TreeNode>>>,
        array: &mut Vec<i32>,
    ) -> bool {
        if let Some(n) = node {
            if n.borrow().val == target.unwrap().borrow().val {
                array.push(n.borrow().val);
                return true;
            }

            if n.borrow().left.is_some() && Self::helper(n.borrow().left.as_ref(), target, array) {
                array.push(n.borrow().val);
                return true;
            }

            if n.borrow().right.is_some() && Self::helper(n.borrow().right.as_ref(), target, array)
            {
                array.push(n.borrow().val);
                return true;
            }
        }
        false
    }
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut parray: Vec<i32> = vec![];
        let mut qarray: Vec<i32> = vec![];

        if p.is_some() && q.is_none() {
            return p;
        } else if p.is_none() && q.is_some() {
            return q;
        }

        Self::helper(root.as_ref(), p.as_ref(), &mut parray);
        Self::helper(root.as_ref(), q.as_ref(), &mut qarray);
        parray.reverse();
        qarray.reverse();

        let min = parray.len().min(qarray.len());

        let mut lca = -1;
        for i in 0..min {
            if parray[i] != qarray[i] {
                break;
            }
            lca = parray[i];
        }
        Some(Rc::new(RefCell::new(TreeNode::new(lca))))
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
        Solution::lowest_common_ancestor(
            tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4],
            Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            Some(Rc::new(RefCell::new(TreeNode::new(1))))
        )
    );
}
