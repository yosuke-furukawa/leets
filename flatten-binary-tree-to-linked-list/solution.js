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
var flatten = function(root, parent) {
  var set = [];
  var helper = (node, parent) => {
    if (node == null) {
      return;
    }
    
    set.push(node);
    // console.log("node", node.val);
    helper(node.left);
    helper(node.right);
  }
  helper(root);
  var head = root;
  if (set.length === 1) {
    return set[0];
  }
  while(set.length > 0) {
    head.left = null;
    head.right = set.shift();
    head = head.right;
  }
};
