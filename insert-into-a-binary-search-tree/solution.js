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
 * @param {number} val
 * @return {TreeNode}
 */
var insertIntoBST = function(root, val) {
  if (root == null) {
    return new TreeNode(val);
  }
  var helper = (node) => {
    if (val < node.val) {
      if (node.left != null) {
        helper(node.left);        
      } else {
        node.left = new TreeNode(val);
      }
    } else {
      if (node.right != null) {
        helper(node.right);        
      } else {
        node.right = new TreeNode(val);
      }
    }
  }
  helper(root);
  return root;
};
