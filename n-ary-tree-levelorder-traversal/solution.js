/**
 * // Definition for a Node.
 * function Node(val,children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */

/**
 * @param {Node} root
 * @return {number[][]}
 */
var levelOrder = function(root) {
  var results = {};
  if (root == null) {
    return [];
  }
  
  var helper = (node, level) => {
    if (node == null) {
      return;
    }
    
    if (results[level]) {
      results[level].push(node.val);
    } else {
      results[level] = [node.val];
    }
    
    node.children.forEach((c) => helper(c, level+1));
  };
  
  helper(root, 0);
  results["length"] = Object.keys(results).length;
  return Array.from(results);
};
