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
var upsideDownBinaryTree = function(root, temp) {
  var current = root;
  var next = null;
  var prev = null;
  var right = null;
  while (current != null) {
    next = current.left;
    current.left = right;
    right = current.right;
    current.right = prev;
    
    prev = current;
    current = next;
  }
  return prev;
};
