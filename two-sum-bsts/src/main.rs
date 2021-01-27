// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

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

impl Solution {
    fn dfs<F: FnMut(i32) -> bool>(node: Option<&Rc<RefCell<TreeNode>>>, consumer: &mut F) {
        if let Some(n) = node {
            let result = consumer(node.as_ref().unwrap().borrow().val);
            if result {
                return;
            }
            Solution::dfs(n.borrow().left.as_ref(), consumer);
            Solution::dfs(n.borrow().right.as_ref(), consumer);
        }
    }

    pub fn two_sum_bs_ts(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>, target: i32) -> bool {
        let mut set = HashSet::new();
        Solution::dfs(root2.as_ref(), &mut (|v| { set.insert(v); false }));
        let mut result = false;
        Solution::dfs(root1.as_ref(), &mut (|v| { result = result || set.contains(&(target - v)); result }));
        result
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::two_sum_bs_ts(tree![2,1,4], tree![1,0,3], 5));
    println!("{}", Solution::two_sum_bs_ts(tree![0,-10,10], tree![5,1,7,0,2], 18));

}
