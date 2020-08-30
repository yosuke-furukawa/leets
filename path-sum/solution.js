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
 * @param {number} sum
 * @return {boolean}
 */
var hasPathSum = function(root, sum) {
  var helper = (node, s, pre) => {
    if (node == null) {
      return false;
    }
    if (node.left == null && node.right == null) {
      return sum === s + node.val;
    }
    var left = helper(node.left, s + node.val)
    if (left) {
      return true;
    }
    var right = helper(node.right, s + node.val)
    if (right) {
      return true;
    }
    return false;
  }
  return helper(root, 0, null);
};
