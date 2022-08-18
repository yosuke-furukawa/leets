use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    fn helper(node: Option<&Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, i32>) -> i32 {
        if let Some(node) = node {
            let n = node.borrow();
            let left = Solution::helper(n.left.as_ref(), map);
            let right = Solution::helper(n.right.as_ref(), map);
            let sum = left + right + n.val;
            *map.entry(sum).or_insert(0) += 1;
            return sum;
        }
        0
    }
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map = HashMap::new();
        Self::helper(root.as_ref(), &mut map);
        let mut max = 0;
        let mut res = vec![];
        for (k, v) in map {
            match v {
                _ if v > max => {
                    max = v;
                    res.clear();
                    res.push(k);
                }
                _ if v == max => res.push(k),
                _ => {}
            }
        }
        res
    }
}

struct Solution;

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
    println!("{:?}", Solution::find_frequent_tree_sum(tree![5, 2, -3]));
    println!("{:?}", Solution::find_frequent_tree_sum(tree![5, 2, -5]));
}
