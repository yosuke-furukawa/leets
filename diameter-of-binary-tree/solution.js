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
var diameterOfBinaryTree = function(root) {
  var max = 0;
  const helper = (node) => {
    if (node == null) {
      return 0;
    }
    var left = 0;
    if (node.left) {
      left = helper(node.left) + 1;      
    }
    var right = 0;
    if (node.right) {
      right = helper(node.right) + 1;
    }
    // console.log({left, right})

    max = Math.max(max, left + right);
    return Math.max(left, right);
  };
  helper(root, 0);
  return max;
};
