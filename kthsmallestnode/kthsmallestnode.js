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
 * @param {number} k
 * @return {number}
 */
var kthSmallest = function(root, k) {
  var result;
  var inorder = function(node) {
    if (k<=0) {
      return;
    }
    if (node.left !== null) {
      inorder(node.left);    
    }
    k--;
    if (k === 0) {
      result = node.val;
      return;
    }
    if (k<0) {
      return;
    }
    if (node.right !== null) {
      inorder(node.right);
    }
  }
  inorder(root);
  return result;
};
