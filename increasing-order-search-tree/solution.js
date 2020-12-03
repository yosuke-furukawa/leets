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
 * @return {TreeNode}
 */
var increasingBST = function(root) {
  var pre = new TreeNode(-1);
  var result = pre;
  var helper = (node) => {
    if (node == null) {
      return;
    }
    helper(node.left);
    pre.right = new TreeNode(node.val);
    pre = pre.right;
    helper(node.right);
  }
  helper(root);
  return result.right;
};
