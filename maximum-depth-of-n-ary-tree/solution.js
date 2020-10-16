/**
 * // Definition for a Node.
 * function Node(val,children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */

/**
 * @param {Node} root
 * @return {number}
 */
var maxDepth = function(root) {
  var max = 0;
  var helper = (node, level) => {
    if (node == null) {
      return;
    }
    if (max < level) {
      max = level;
    }
    node.children.forEach((child) => helper(child, level+1));
  }
  helper(root, 1);
  return max;
};
