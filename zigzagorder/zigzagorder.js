/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */

var zigzag = function(node, level, result) {
  if (node === null) {
    return;
  }
  if (result[level] == null) {
    result[level] = [node.val];
  } else {
    if (level % 2 === 0) {
      result[level].push(node.val);
    } else {
      result[level].unshift(node.val);
    }
  }
  
  zigzag(node.left, level+1,result);
  zigzag(node.right, level+1,result);
  
}
/**
 * @param {TreeNode} root
 * @return {number[][]}
 */
var zigzagLevelOrder = function(root) {
  var result = [];
  zigzag(root, 0, result);
  return result;
};
