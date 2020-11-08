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
var findTilt = function(root) {
  var sum = 0;
  var helper = (node) => {
    if (node == null) {
      return 0;
    }
    var l = helper(node.left);
    var r = helper(node.right);
    sum += Math.abs(l-r);
    return node.val + l + r;
  }
  helper(root, 0);
  return sum;
};
