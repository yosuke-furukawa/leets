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
var pseudoPalindromicPaths  = function(root) {
  var result = 0;
  var helper = (node, paths) => {
    if (paths.has(node.val)) {
      paths.set(node.val, paths.get(node.val) + 1);
    } else {
      paths.set(node.val, 1);
    }
    
    if (node.left == null && node.right == null) {
      var oddCount = 0;
      for (var val of paths.values()) {
        if (val % 2 === 1) {
          oddCount++;
          if (oddCount > 1) {
            return;
          }
        }
      }
      result++;
      return;
    }
    
    if (node.left != null) {
      helper(node.left, new Map(paths));      
    }
 
    if (node.right != null) {
      helper(node.right, new Map(paths));
    }
  }
  var map = new Map();
  helper(root, map);
  return result;
};
