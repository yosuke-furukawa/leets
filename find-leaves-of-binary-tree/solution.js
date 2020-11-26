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
 * @return {number[][]}
 */
var findLeaves = function(root) {
  var findLevel = (node, level) => {
    if (node == null) {
      return level;
    }
    return Math.max(findLevel(node.left, level+1), findLevel(node.right, level+1));
  }
  var level = findLevel(root, 0);
  // console.log(level);
  var helper = (node, parent, direction, result) => {
    if (node == null) {
      return;
    }
    if (node.left == null && node.right == null) {
      result.push(node.val);
      if (parent != null) {
        parent[direction] = null;        
      }
    }
    
    helper(node.left, node, "left", result);
    helper(node.right, node, "right", result);
  }
  
  var result = [];
  for (var i=0;i<level;i++) {
    var r = []
    helper(root, null, "", r);
    result.push(r);
  }
  return result;
};
