/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} original
 * @param {TreeNode} cloned
 * @param {TreeNode} target
 * @return {TreeNode}
 */

var getTargetCopy = function(original, cloned, target) {
  var helper = (clonedNode) => {
    if (clonedNode == null) {
      return;
    }
    if (clonedNode.val === target.val) {
      return clonedNode;
    }
    var l = helper(clonedNode.left);
    var r = helper(clonedNode.right);
    return l || r;
  }
  var result = helper(cloned);
  return result;
};
