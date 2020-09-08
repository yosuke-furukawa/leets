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
 * @return {string[]}
 */
var binaryTreePaths = function(root) {
  var result = [];
  var helper = (node, arr) => {
    if (node == null) {
      return;
    }
    
    arr.push(node.val);
    helper(node.left, [...arr]);
    helper(node.right, [...arr]);
    if (node.left == null && node.right == null) {
      result.push(arr.join("->"));
    }
  };
  helper(root, []);
  return result;
};
