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
 * @return {string[][]}
 */
var printTree = function(root) {
  var getDepth = (node, level) => {
    if (node == null) {
      return level-1;
    }
    
    var l = getDepth(node.left, level+1);
    var r = getDepth(node.right, level+1);
    return Math.max(l, r);
  }
  var depth = getDepth(root, 0);
  var matrix = new Array(depth+1);
  var len = 2**(depth+1)-1;
  for (var i=0;i<matrix.length;i++) {
    matrix[i] = new Array(len).fill("");
  }
  var helper = (node, xpos, ypos, len) => {
    if (node == null) {
      return;
    }

    matrix[xpos][ypos] = "" + node.val;
    helper(node.left, xpos+1, ypos - Math.ceil(len/2), Math.floor(len/2));
    helper(node.right, xpos+1, ypos + Math.ceil(len/2), Math.floor(len/2));
  }
  helper(root, 0, Math.floor(len/2), Math.floor(len/2));
  return matrix;
};
