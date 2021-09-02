use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn generate(
        start: usize,
        end: usize,
        dp: &mut Vec<Vec<Vec<Option<Rc<RefCell<TreeNode>>>>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if start > end {
            return vec![None];
        }

        if !dp[start][end].is_empty() {
            return dp[start][end].clone();
        }

        let mut res = vec![];

        for root in start..=end {
            for left in Self::generate(start, root - 1, dp) {
                for right in Self::generate(root + 1, end, dp) {
                    let mut tree = TreeNode::new(root as i32);
                    tree.left = left.clone();
                    tree.right = right.clone();
                    res.push(Some(Rc::new(RefCell::new(tree))));
                }
            }
        }

        dp[start][end] = res.clone();
        res
    }
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut dp = vec![vec![vec![]; n as usize + 1]; n as usize + 1];
        Self::generate(1, n as usize, &mut dp)
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
    let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
    let mut queue = std::collections::VecDeque::new();
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

fn main() {
    println!("{:?}", Solution::generate_trees(3));
}
