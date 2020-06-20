/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */

var inorder = function(node, result) {
  if (node === null) {
    return;
  }
  inorder(node.left, result);
  
  result.push(node.val);
  
  inorder(node.right, result);
}
/**
 * @param {TreeNode} root
 * @return {number[]}
 */
var inorderTraversal = function(root) {
  var result = [];
  inorder(root, result);
  return result;
};
