/**
 * // Definition for a Node.
 * function Node(val, children) {
 *    this.val = val === undefined ? 0 : val;
 *    this.children = children === undefined ? [] : children;
 * };
 */

/**
 * @param {Node[]} tree
 * @return {Node}
 */
var findRoot = function(tree) {
  var set = new Set();
  var array = [];
  for (var node of tree) {
    if (node.children.length > 0) {
      for (var child of node.children) {
        set.add(child.val);
      }
    }
  }
  
  for (var node of tree) {
    if (!set.has(node.val)) {
      return node;
    }
  }
  return null;
};
