use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    fn traverse(
        left: usize,
        right: usize,
        map: &HashMap<i32, usize>,
        pre_index: &mut i32,
        preorder: &[i32],
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if left == right {
            return None;
        }
        let root_val = preorder[*pre_index as usize];
        let mut root = TreeNode::new(root_val);
        let index = map.get(&root_val).unwrap();

        *pre_index += 1;
        root.left = Self::traverse(left, *index, map, pre_index, preorder);
        root.right = Self::traverse(*index + 1, right, map, pre_index, preorder);
        Some(Rc::new(RefCell::new(root)))
    }
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map = HashMap::new();
        for (i, v) in inorder.iter().cloned().enumerate() {
            map.insert(v, i);
        }
        Self::traverse(0, inorder.len(), &map, &mut 0, &preorder)
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

fn main() {
    println!(
        "{:?}",
        Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
    );
}
