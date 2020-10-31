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
 * @return {void} Do not return anything, modify root in-place instead.
 */
var recoverTree = function(root) {
  var trees = [];
  var helper = (node) => {
    if (node == null) {
      return;
    }
    helper(node.left);
    trees.push(node.val);
    helper(node.right);
  };
  helper(root);
  trees.sort((a, b) => a-b);
  // console.log(trees);
  var fix = (node) => {
    if (node == null) {
      return;
    }
    fix(node.left);
    let f = trees.shift();
    if (node.val !== f) {
      node.val = f;
    }
    fix(node.right);
  };
  fix(root);
};
