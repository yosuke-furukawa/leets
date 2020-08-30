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
var countUnivalSubtrees = function(root) {
  var count = 0;
  if (root == null) {
    return count;
  }
  var helper = (node) => {
    if (node.left == null && node.right == null) {
      count++;
      return true;
    }
    var isUnival = true;
    
    if (node.left != null) {
      isUnival = helper(node.left) && isUnival && node.left.val == node.val;
    }
    if (node.right != null) {
      isUnival = helper(node.right) && isUnival && node.right.val == node.val;
    }
    
    if (!isUnival) {
      return false;
    }
    count++;
    return true;
  }
  helper(root);
  return count;
};
