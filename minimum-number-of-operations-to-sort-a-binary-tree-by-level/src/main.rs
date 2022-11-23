use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn helper(node: Option<&Rc<RefCell<TreeNode>>>, nums: &mut Vec<Vec<i32>>, level: usize) {
        if let Some(n) = node {
            let node = n.borrow();
            if nums.len() <= level {
                nums.push(vec![]);
            }
            nums[level].push(node.val);
            Self::helper(node.left.as_ref(), nums, level + 1);
            Self::helper(node.right.as_ref(), nums, level + 1);
        }
    }
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut nodes = vec![];
        Self::helper(root.as_ref(), &mut nodes, 0);
        let mut counts = 0;
        for nums in nodes {
            let mut sorted = nums
                .iter()
                .enumerate()
                .map(|(i, num)| (num, i))
                .collect::<Vec<_>>();
            sorted.sort();
            let mut count = 0;
            let mut visited = vec![false; nums.len()];
            for i in 0..nums.len() {
                if visited[i] || sorted[i].1 == i {
                    continue;
                }
                let mut j = i;
                let mut cycle = 0;
                while !visited[j] {
                    visited[j] = true;
                    j = sorted[j].1;
                    cycle += 1;
                }
                if cycle > 0 {
                    count += cycle - 1;
                }
            }
            counts += count;
        }
        counts
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
        Solution::minimum_operations(tree![
            1, 4, 3, 7, 6, 8, 5, null, null, null, null, 9, null, 10
        ])
    );
    println!(
        "{}",
        Solution::minimum_operations(tree![49, 45, 1, 20, 46, 15, 39, 27, null, null, null, 25])
    );
}
