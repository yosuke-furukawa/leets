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
 * @param {number} sum
 * @return {number}
 */
var pathSum = function(root, sum) {
  var count = 0;
  var helper = (node, s, isroot) => {
    if (node == null) {
      return;
    }
    s -= node.val;
    if (s === 0) {
      count++;
    }
    helper(node.left, s, false);
    helper(node.right, s, false);
    if (isroot) {
      helper(node.left, sum, true);
      helper(node.right, sum, true);
    }
  };
  helper(root, sum, true);
  
  return count;
};
