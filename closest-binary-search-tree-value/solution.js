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
 * @param {number} target
 * @return {number}
 */
var closestValue = function(root, target) {
  var closest = root.val;
  var diff = Math.abs(target - closest);
  var helper = (node) => {
    if (node === null) {
      return;
    }
    var d = Math.abs(node.val - target);
    if (d < diff) {
      closest = node.val;
      diff = d;
    }
    if (target - node.val < 0) {
      helper(node.left);
    } else if (target - node.val > 0) {
      helper(node.right);
    } else {
      return;
    }
  };
  helper(root);
  return closest;
};
