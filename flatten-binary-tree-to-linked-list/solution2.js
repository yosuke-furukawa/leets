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
 * @return {void} Do not return anything, modify root in-place instead.
 */
var flatten = function(root) {
  if (root == null) {
    return;
  }

  var node = root;

  while (node != null) {

    // If the node has a left child
    if (node.left != null) {

      // Find the rightmost node
      var rightmost = node.left;
      while (rightmost.right != null) {
        rightmost = rightmost.right;
      }

      // rewire the connections
      rightmost.right = node.right;
      node.right = node.left;
      node.left = null;
    }

    // move on to the right side of the tree
    node = node.right;
  }
};
