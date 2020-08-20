/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {void} Do not return anything, modify head in-place instead.
 */
var reorderList = function(head) {
  var stack = [];
  var h = head;
  while(h != null) {
    stack.unshift(h);
    h = h.next;
  }
  
  h = head;
  while(h != null) {
    var l = stack.shift();
    var n = h.next;
    if (l != null) {
      h.next = l;
      l.next = n;
    }
    
    if (l === h) {
      h.next = null;
      break;
    }

    h = n;
    if (l === n) {
      h.next = null;
      break;
    }
  }
};
