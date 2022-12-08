use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn helper(node: Option<&Rc<RefCell<TreeNode>>>, arrays: &mut Vec<i32>) {
        if let Some(node) = node {
            let n = node.borrow();
            if n.left.is_none() && n.right.is_none() {
                arrays.push(n.val);
            }
            Self::helper(n.left.as_ref(), arrays);
            Self::helper(n.right.as_ref(), arrays);
        }
    }
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut array1 = vec![];
        Self::helper(root1.as_ref(), &mut array1);
        let mut array2 = vec![];
        Self::helper(root2.as_ref(), &mut array2);
        array1.len() == array2.len() && array1.iter().zip(array2.iter()).all(|(a, b)| a == b)
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
        "{}",
        Solution::leaf_similar(
            tree![3, 5, 1, 6, 2, 9, 8, null, null, 7, 4],
            tree![3, 5, 1, 6, 7, 4, 2, null, null, null, null, null, null, 9, 8]
        )
    );
    println!("{}", Solution::leaf_similar(tree![1, 2, 3], tree![1, 3, 2]));
}
