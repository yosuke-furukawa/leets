/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @param {TreeNode} p
 * @return {TreeNode}
 */
var inorderSuccessor = function(root, p) {
  var found = null;
  var preval = -1;
  var maxval = Infinity;
  var inorder = function(node) {
    if (node == null) {
      return;
    }
    if (found != null) {
      return;
    }
    if (node.left) {
      inorder(node.left);
    }
    if (maxval < node.val && found == null) {
      maxval = node.val;
      found = node;
    }
    if (preval === p.val && found == null) {
      if (node.val > p.val) {
        found = node;
      }
      maxval = p.val;
    }
    preval = node.val;
    if (node.right) {
      inorder(node.right);
    }
  }
  inorder(root);
  return found;
};
