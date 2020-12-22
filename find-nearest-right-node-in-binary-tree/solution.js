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
 * @param {TreeNode} u
 * @return {TreeNode}
 */
var findNearestRightNode = function(root, u) {
  var target;
  var map = {};
  var helper = (node, level) => {
    if (node == null) {
      return;
    }
    
    if (map[level] != null) {
      map[level].push(node.val);
    } else {
      map[level] = [node.val];
    }
    
    if (node.val === u.val) {
      target = map[level];
    }

    helper(node.left, level+1);
    helper(node.right, level+1);
  };
  helper(root, 0);
  var value = target[target.indexOf(u.val)+1] ?? null;
  if (value != null) {
    return new TreeNode(value);
  } else {
    return value;
  }
};
