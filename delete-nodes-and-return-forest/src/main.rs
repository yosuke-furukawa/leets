use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

impl Solution {
    fn dfs(
        node: Option<&Rc<RefCell<TreeNode>>>,
        to_delete: HashSet<i32>,
        array: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        is_root: bool,
    ) {
        if let Some(n) = node {
            let mut v = n.borrow_mut();
            if !to_delete.contains(&(v.val)) && is_root {
                array.push(Some(n.clone()));
            }

            let is_root = to_delete.contains(&(v.val));
            if v.left != None {
                Self::dfs(v.left.as_ref(), to_delete.clone(), array, is_root);
                if to_delete.contains(&(v.left.as_ref().unwrap().borrow().val)) {
                    v.left = None;
                }
            }

            if v.right != None {
                Self::dfs(v.right.as_ref(), to_delete.clone(), array, is_root);
                if to_delete.contains(&(v.right.as_ref().unwrap().borrow().val)) {
                    v.right = None;
                }
            }
        }
    }

    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut ans = vec![];
        Self::dfs(
            root.as_ref(),
            to_delete.into_iter().collect(),
            &mut ans,
            true,
        );
        ans
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

fn main() {
    println!(
        "{:?}",
        Solution::del_nodes(tree![1, 2, 3, 4, 5, 6, 7], vec![3, 5])
    );
}
