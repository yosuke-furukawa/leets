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
 * @return {boolean}
 */
var isBalanced = function(root) {
  const helper = (node, level) => {
    if (node == null) {
      return [0, true];
    }
    
    var left = helper(node.left, level+1);
    var right = helper(node.right, level+1);
    return [Math.max(left[0], right[0]) +1, left[1] && right[1] && Math.abs(left[0] - right[0]) <= 1] ;
  }
  const result = helper(root, 0);
  return result[1];
};
