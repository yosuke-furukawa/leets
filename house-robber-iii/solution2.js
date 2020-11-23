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
var rob = function(root) {
  var money1 = 0;
  var money2 = 0;
  var helper = (node) => {
    if (node == null) {
      return [0, 0];
    }
    
    var left = helper(node.left);
    var right = helper(node.right);
    var rob =  node.val + left[1] + right[1];
    var notRob = Math.max(left[0], left[1]) + Math.max(right[0], right[1]);
    return [rob, notRob];
  }
  
  var robs = helper(root);
  return Math.max(robs[0], robs[1]);
};
