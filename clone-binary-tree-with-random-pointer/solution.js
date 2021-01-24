/**
 * // Definition for a Node.
 * function Node(val, left, right, random) {
 *    this.val = val === undefined ? null : val;
 *    this.left = left === undefined ? null : left;
 *    this.right = right === undefined ? null : right;
 *    this.random = random === undefined ? null : random;
 * };
 */

/**
 * @param {Node} root
 * @return {NodeCopy}
 */
var copyRandomBinaryTree = function(root) {
  var map = new Map();
  var helper = (from, to) => {
    if (from == null) {
      return null;
    }
    if (map.has(from)) {
      return map.get(from);
    }
    
    map.set(from, to);
    to.val = from.val;
    if (from.left) {
      to.left = helper(from.left, new NodeCopy());
    }
    if (from.right) {
      to.right = helper(from.right, new NodeCopy());
    }
    if (from.random) {
      to.random = helper(from.random, new NodeCopy());
    }

    return to;
  };
  return helper(root, new NodeCopy());
};
