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
var verticalTraversal = function(root) {
  if (root == null) {
    return [];
  } 
  var array = [];
  var minX = 0, maxX = 0;
  var dfs = (node, x, y) => {
    if (node === null) {
      return null;
    }
    
    dfs(node.left, x-1, y+1);
    minX = Math.min(x, minX);
    maxX = Math.max(x, maxX);
    array.push({x, y, v: node.val});
    dfs(node.right, x+1, y+1);
  };
  dfs(root, 0, 0);
  var result = new Array(maxX - minX + 1);
  array.sort((a, b) => { 
    var compX = a.x - b.x;
    if (compX === 0) {
      var compY = a.y - b.y;
      if (compY === 0) {
        return a.v - b.v;
      }
      return compY;
    }
    return compX;
  });
  for (const node of array) {
    if (result[node.x-minX]) {
      result[node.x-minX].push(node.v);
    } else {
      result[node.x-minX] = [node.v];
    }
  }
  
  return result;
};
