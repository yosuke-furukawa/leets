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
var sumRootToLeaf = function(root) {
  var sum = 0;
  var helper = (node, values) => {
    if (node == null) {
      return;
    }
    values += node.val;
    helper(node.left, values);
    helper(node.right, values);
    if (node.left == null && node.right == null) {
      sum += Number.parseInt(values, 2);
    }
  };
  helper(root, "");
  return sum;
};
