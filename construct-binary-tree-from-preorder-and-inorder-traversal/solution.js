/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */

/**
 * @param {number[]} preorder
 * @param {number[]} inorder
 * @return {TreeNode}
 */
var buildTree = function(preorder, inorder) {
  var preIdx = 0;
  var map = new Map();
  var traverse = function(leftIdx, rightIdx) {
    if (leftIdx === rightIdx) {
      return null;
    }
    var rootVal = preorder[preIdx];
    var root = new TreeNode(rootVal);
    var index = map.get(rootVal);
    
    preIdx++;
    root.left = traverse(leftIdx, index);
    root.right = traverse(index+1, rightIdx);
    return root;
  }
  
  for (var i=0;i<inorder.length;i++) {
    map.set(inorder[i], i);
  }
  return traverse(0, inorder.length);
};
