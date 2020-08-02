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
var countNodes = function(root) {
  var count = 0;
  var helper = function(node) {
    if (node == null) {
      return null;
    }
    node.val != null && count++;
    helper(node.left)
    helper(node.right);
  }
  helper(root);
  return count;
};
