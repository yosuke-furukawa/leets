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
var verticalOrder = function(root) {
  var map = {};
  var result = [];
  var helper = (node, vertical, level) => {
    if (node == null) {
      return;
    }
    if (map[vertical] != null) {
      var list = map[vertical];
      list.push({val: node.val, level});
    } else {
      map[vertical] = [{val: node.val, level}];      
    }
    helper(node.left, vertical+1, level+1);
    helper(node.right, vertical-1, level+1);
  }
  helper(root, 0, 0);
  for (var key of Object.keys(map).sort((a, b) => b-a)) {
    result.push(map[key].sort((ma, mb) => ma.level - mb.level).map((m) => m.val));
  }
  return result;
};
