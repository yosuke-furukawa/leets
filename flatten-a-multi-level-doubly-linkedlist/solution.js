/**
 * // Definition for a Node.
 * function Node(val,prev,next,child) {
 *    this.val = val;
 *    this.prev = prev;
 *    this.next = next;
 *    this.child = child;
 * };
 */

var traverse = function(node, next) {
  let n = node;
  while (n?.next != null || n?.child != null) {
    if (n.child) {
      const { next, child } = n;
      n.next = child;
      child.prev = n;
      n.child = null;
      traverse(child, next);
      if (next) n = next.prev;
    }
    n = n.next;
  }
  if (next) {
    n.next = next;
    next.prev = n;
  }
} 

/**
 * @param {Node} head
 * @return {Node}
 */
var flatten = function(head) {
  var h = head;
  if (h == null) {
    return null;
  }
  traverse(h, null);
  return head;
};
