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
  var queueOriginal = [original];
  var queueCloned = [cloned];
  while (queueOriginal.length > 0) {
    var nodeOriginal = queueOriginal.pop();
    var nodeCloned = queueCloned.pop();
    if (nodeOriginal == target) {
      return nodeCloned;
    }
    
    if (nodeOriginal.left != null) {
      queueOriginal.push(nodeOriginal.left);
      queueCloned.push(nodeCloned.left);
    }
    
    if (nodeOriginal.right != null) {
      queueOriginal.push(nodeOriginal.right);
      queueCloned.push(nodeCloned.right);
    }
  }
  
  return null;
};
