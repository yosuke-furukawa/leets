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
  var robResult = new Map();
  var notRobResult = new Map();
  var helper = (node, parentRobbed) => {
    if (node == null) {
      return 0;
    }
    
    if (parentRobbed) {
      if (robResult.has(node)) {
        return robResult.get(node);
      }
      var result = helper(node.left, false) + helper(node.right, false);
      robResult.set(node, result);
      return result;
    } else {
      if (notRobResult.has(node)) {
        return notRobResult.get(node);
      }
      var rob = node.val + helper(node.left, true) + helper(node.right, true);
      var notRob = helper(node.left, false) + helper(node.right, false);
      var result = Math.max(rob, notRob);
      notRobResult.set(node, result);
      return result;
    }
  }
  
  var robs = helper(root, false);
  return robs;
};
