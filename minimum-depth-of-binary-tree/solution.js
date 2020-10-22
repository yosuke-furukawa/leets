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
var minDepth = function(root) {
  if (root == null) {
    return 0;
  }
  var helper = (node, level) => {
    if (node == null) {
      return level;
    }
    // console.log({ node, level })

    var l = Infinity;
    if (node.left != null) {
      l = helper(node.left, level+1);
    }
    
    var r = Infinity;
    if (node.right != null) {
      r = helper(node.right, level+1);  
    }
    if (l === Infinity && r === Infinity) {
      return level;
    }
    return Math.min(l, r);
  }
  return helper(root, 1);
};
