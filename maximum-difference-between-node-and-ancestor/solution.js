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
var maxAncestorDiff = function(root) {
  var maxDiff = -1;
  var helper = (node, min, max) => {
    if (node == null) {
      return;
    }
    if (node.val > max) {
      max = node.val;
    }
    if (node.val < min) {
      min = node.val;
    }
    maxDiff = Math.max(maxDiff, Math.abs(max - min));
    helper(node.left, min, max);
    helper(node.right, min, max);
    
  }
  helper(root, Infinity, -1);
  return maxDiff;
};
