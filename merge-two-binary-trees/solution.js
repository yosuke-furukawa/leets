/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} t1
 * @param {TreeNode} t2
 * @return {TreeNode}
 */
var mergeTrees = function(t1, t2) {
  if (t1 == null && t2 == null) {
    return null;
  }
  var helper = (node, n1, n2) => {
    if (n1 === null && n2 === null) {
      return;
    }
    var v1 = n1?.val ?? 0;
    var v2 = n2?.val ?? 0;
    node.val = v1 + v2;
    if (n1?.left != null || n2?.left != null) {
      node.left = new TreeNode();
      helper(node.left, n1?.left, n2?.left);      
    }
    
    if (n1?.right != null || n2?.right != null) {
      node.right = new TreeNode();
      helper(node.right, n1?.right, n2?.right);      
    }
  }
  var root = new TreeNode();
  helper(root, t1, t2);
  return root;
};
