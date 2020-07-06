/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
var traverse = (node, target, parents) => {
  if (node == null) {
    return;
  }

  if (node.val === target.val) {
    parents.unshift(node);
    return true;
  }
  
  if (node.left != null) {
    if (traverse(node.left, target, parents)) {
      parents.unshift(node);
      return true;
    }
  }
  if (node.right != null) {
    if (traverse(node.right, target, parents)) {
      parents.unshift(node);
      return true;
    }
  }

};
/**
 * @param {TreeNode} root
 * @param {TreeNode} p
 * @param {TreeNode} q
 * @return {TreeNode}
 */
var lowestCommonAncestor = function(root, p, q) {
  var pparents = [];
  var qparents = [];
  if (p != null && q == null) {
    return p;
  } else if (p == null && q != null) {
    return q;
  }
  traverse(root, p, pparents);
  traverse(root, q, qparents);
  var len = Math.min(pparents.length, qparents.length);
  for (var i=0; i<len;i++) {
    if (pparents[i].val !== qparents[i].val) {
      break;
    }
  }
  return pparents[i-1];
};
