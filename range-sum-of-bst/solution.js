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
 * @param {number} low
 * @param {number} high
 * @return {number}
 */
var rangeSumBST = function(root, low, high) {
  var sum = 0;
  var helper = (node) => {
    if (node == null) {
      return;
    }
    
    if (node.val >= low && node.val <= high) {
      sum += node.val;
    }
    helper(node.left);
    helper(node.right);
  }
  
  helper(root);
  return sum;
};
