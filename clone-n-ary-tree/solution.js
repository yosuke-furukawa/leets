/**
 * // Definition for a Node.
 * function Node(val, children) {
 *    this.val = val === undefined ? 0 : val;
 *    this.children = children === undefined ? [] : children;
 * };
 */

/**
 * @param {Node} node
 * @return {Node}
 */
var cloneTree = function(root) {
  var helper = (from, to) => {
    if (from == null) {
      return;
    }
    to.val = from.val;
    for (var child of from.children) {
      to.children.push(helper(child, new Node()));
    }
    return to;
  }
  return helper(root, new Node());
};
