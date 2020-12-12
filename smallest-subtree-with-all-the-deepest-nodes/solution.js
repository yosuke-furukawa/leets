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
 * @return {TreeNode}
 */
var subtreeWithAllDeepest = function(root) {
  var depth = 0;
  var result = root;
  var helper = (node, level) => {
    if (node == null) {
      return level-1;
    }
    var l = helper(node.left, level+1);
    var r = helper(node.right, level+1);
    if (l === r && l >= depth) {
      depth = l;
      result = node;
    }
    return Math.max(l, r);
  };
  
  helper(root, 0);
  return result;
};
