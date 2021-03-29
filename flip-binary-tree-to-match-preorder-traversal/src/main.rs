use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    fn helper(node: Option<&Rc<RefCell<TreeNode>>>, vi: &mut usize, voyage: &Vec<i32>, result: &mut Vec<i32>) {
        // println!("{:?}, {:?}", node, voyage[*vi]);
        if let Some(v) = result.first() {
            if v == &-1 {
                return;
            }
        }
        if let Some(n) = node {
            let n = n.as_ref().borrow();
            *vi += 1;
            if n.val != voyage[*vi - 1] {
                *result = vec![-1];
            } else if n.left.is_some() && n.left.as_ref().unwrap().borrow().val != voyage[*vi]  {
                result.push(n.val);
                Self::helper(n.right.as_ref(), vi, voyage, result);
                Self::helper(n.left.as_ref(), vi, voyage, result);
            } else {
                Self::helper(n.left.as_ref(), vi, voyage, result);
                Self::helper(n.right.as_ref(), vi, voyage, result);
            }
        }
    }

    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        let mut results = vec![];
        Self::helper(root.as_ref(), &mut 0, &voyage, &mut results);
        results
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
    println!("{:?}", Solution::flip_match_voyage(tree![1,2,3,4,5,6,7], vec![1,2,4,5,3,6,7]));
    println!("{:?}", Solution::flip_match_voyage(tree![1,2,3,6,7,4,5], vec![1,3,2,4,5,7,6]));
    println!("{:?}", Solution::flip_match_voyage(tree![1,2,null,3], vec![1,3,2]));
    println!("{:?}", Solution::flip_match_voyage(tree![1,2,null,3], vec![1,2,3]));
    println!("{:?}", Solution::flip_match_voyage(tree![1,null,2,null,3], vec![1,3,2]));
    println!("{:?}", Solution::flip_match_voyage(tree![1,null,2,null,3], vec![1,2,3]));
    println!("{:?}", Solution::flip_match_voyage(tree![2,1,4,5,null,3], vec![2,4,3,1,5]));
}
