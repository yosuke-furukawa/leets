/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
var bfs = function(left, right, result, depth) {
  if (left === null && right === null) {
    return;
  }
  if (!result[depth]) {
    result[depth] = [];
  }
  if (left !== null) {
    result[depth].push(left.val);
  }
  if (right !== null) {
    result[depth].push(right.val);
  }
  left && bfs(left.left, left.right, result, depth+1);
  right && bfs(right.left, right.right, result, depth+1);
};

/**
 * @param {TreeNode} root
 * @return {number[][]}
 */
var levelOrderBottom = function(root) {
  if (!root) {
    return [];
  }
  var result = [[root.val]];
  var depth = 1;
  bfs(root.left, root.right, result, depth);
  return result.reverse();
};
