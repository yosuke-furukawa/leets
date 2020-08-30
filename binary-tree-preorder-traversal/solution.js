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
 * @return {number[]}
 */
var preorderTraversal = function(root) {
  var num = [];
  var helper = (node) => {
    if (node == null) {
      return;
    }
    num.push(node.val);
    helper(node.left);
    helper(node.right);
  }
  helper(root);
  return num;
};
