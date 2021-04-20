/**
 * // Definition for a Node.
 * function Node(val, children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */

/**
 * @param {Node} root
 * @return {number[]}
 */
var preorder = function(root) {
  var results = [];
  var helper = (node) => {
    if (!node || !node.children) {
      return;
    }
    results.push(node.val);
    node.children.forEach((n) => helper(n));
  }
  helper(root);
  return results;
};
