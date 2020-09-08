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
 * @return {number[]}
 */
var rightSideView = function(root) {
  var array = [];
  var added = new Set();
  var helper = (node, level) => {
    if (node == null) {
      return;
    }
    
    if (!added.has(level)) {
      array.push(node.val);
      added.add(level);
    }
    helper(node.right, level+1);
    helper(node.left, level+1);
  };
  helper(root, 1);
  return array;
};
