/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */

var size = function(node) {
  var s = 0;
  while (node != null) {
    s++;
    node = node.next;
  }
  return s;
}

/**
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
var addTwoNumbers = function(l1, l2) {
  var s1 = size(l1);
  var s2 = size(l2);
  var s = Math.abs(s1 - s2);
  if (s > 0) {
    var ln = s1 < s2 ? l1 : l2;
    while (s > 0) {
      ln = new ListNode(0, ln);
      s--;
    }
    if (s1 < s2) {
      l1 = ln;
    } else {
      l2 = ln;
    }
  }
  
  var li1 = l1;
  var li2 = l2;
  var helper = (l1, l2) => {
    if (l1.next == null || l2.next == null) {
      const ans = l1.val + l2.val;
      const m = Math.floor(ans / 10);
      const a = ans % 10;
      const node = new ListNode(a);
      return {node, m};
    } else if (l1.next && l2.next) {
      const {node, m} = helper(l1.next, l2.next);
      const ans = l1.val + l2.val + m;
      const moveup = Math.floor(ans / 10);
      const a = ans % 10;
      const newNode = new ListNode(a, node);
      return {node: newNode, m: moveup};
    }
  }
  var {node, m} = helper(li1, li2);
  if (m > 0) {
    node = new ListNode(m, node);
  }
  return node;
};
