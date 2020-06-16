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
 * @return {TreeNode}
 */
var bstFromPreorder = function(preorder) {
    var root = new TreeNode(preorder[0]);
    for (var i=1;i<preorder.length;i++) {
        var curr = root;
        var node = new TreeNode(preorder[i]);
        while(curr != node) {
          if (curr.val >= preorder[i]) {
            if (!curr.left) {
              curr.left = node;
            }
            curr = curr.left;
          } else {
            if (!curr.right) {
              curr.right = node;
            }
            curr = curr.right;
          }
        }
    }
    return root;
};
