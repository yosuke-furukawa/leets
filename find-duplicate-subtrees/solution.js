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
 * @return {TreeNode[]}
 */
var findDuplicateSubtrees = function(root) {
  var map = new Map();
  var result = [];
  var helper = (node) => {
    if (node == null) {
      return "";
    }
    var str = "(";
    str += helper(node.left);
    str += node.val;
    str += ",";
    str += helper(node.right);
    str += ")";
    // console.log(str);
    if (map.has(str) && map.get(str) === 1) {
      result.push(node);
    }
    
    if (map.has(str)) {
      map.set(str, map.get(str)+1);
    } else {
      map.set(str, 1);
    }
    return str;
  };
  helper(root);
  return result;
};
