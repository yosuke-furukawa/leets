/**
 * // Definition for a Node.
 * function Node(val,children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */

/**
 * @param {Node} root
 * @return {number[]}
 */
var postorder = function(root) {
  if (root == null) {
    return [];
  }
  var results = [];
  var helper = (node) => {
    if (node.children.length === 0) {
      results.push(node.val);
      return;
    }
    
    node.children.forEach((n) => helper(n));
    results.push(node.val);
  };
  helper(root);
  return results;
};
