/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {number[]} inorder
 * @param {number[]} postorder
 * @return {TreeNode}
 */
var buildTree = function(inorder, postorder) {
  var postIdx = postorder.length-1;
  var map = new Map();
  var traverse = function(leftIdx, rightIdx) {
    if (rightIdx < leftIdx) {
      return null;
    }
   
    var rootVal = postorder[postIdx];
    var root = new TreeNode(rootVal);
    var index = map.get(rootVal);
    
    postIdx--;
    root.right = traverse(index+1, rightIdx);
    root.left = traverse(leftIdx, index-1);

    return root;
  }
  
  for (var i=0;i<inorder.length;i++) {
    map.set(inorder[i], i);
  }
  return traverse(0, inorder.length-1);

};
