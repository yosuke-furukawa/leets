/**
 * // Definition for a Node.
 * function Node(val, left, right, next) {
 *    this.val = val === undefined ? null : val;
 *    this.left = left === undefined ? null : left;
 *    this.right = right === undefined ? null : right;
 *    this.next = next === undefined ? null : next;
 * };
 */

/**
 * @param {Node} root
 * @return {Node}
 */
var connect = function(root) {
  if (!root) return root;
  var queue = [];
  queue.unshift({node: root, depth:0});
  while(queue.length >= 1) {
    var entry = queue.pop();
    var { node, depth } = entry;
    if (queue[queue.length-1]?.depth === depth) {
      node.next = queue[queue.length-1].node;
    }
    if (node.left != null) {
      queue.unshift({node: node.left, depth: depth + 1});
    }
    if (node.right != null) {
      queue.unshift({node: node.right, depth: depth + 1});
    }
  }
  return root;
};
