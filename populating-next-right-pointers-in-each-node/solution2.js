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
  if (root == null) {
    return root;
  }
        
  var leftmost = root;
  while (leftmost.left != null) {
    var head = leftmost;
    while (head != null) {
      head.left.next = head.right;
      if (head.next != null) {
        head.right.next = head.next.left;
      }
      head = head.next;
    }
    leftmost = leftmost.left;
  }
  return root;
};
