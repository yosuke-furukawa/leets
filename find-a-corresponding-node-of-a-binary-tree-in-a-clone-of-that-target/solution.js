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
  var helper = (originalNode, clonedNode) => {
    if (originalNode == null) {
      return;
    }
    if (originalNode.val === target.val) {
      return clonedNode;
    }
    var l = helper(originalNode.left, clonedNode.left);
    var r = helper(originalNode.right, clonedNode.right);
    return l || r;
  }
  var result = helper(original, cloned);
  return result;
};
