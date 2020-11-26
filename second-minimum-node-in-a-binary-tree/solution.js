/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number}
 */
var findSecondMinimumValue = function(root) {
  var candidate = Infinity;
  var min = root.val;
  var helper = (node) => {
    if (node == null) {
      return;
    }
    
    if (node.left == null && node.right == null) {
      return;
    }
    var c = Math.max(node.left.val, node.right.val);
    if (c !== min) {
      candidate = Math.min(candidate, c);
    }
    if (node.left.val == node.val) {
      helper(node.left);
    }
    if (node.right.val == node.val) {
      helper(node.right);
    }
  }
  
  helper(root);
  if (candidate == Infinity || candidate === root.val) {
    return -1;
  }
  return candidate;
};
