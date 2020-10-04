/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */

/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var detectCycle = function(head) {
  var slow = head;
  var fast = head;
  do {
    slow = slow?.next;
    fast = fast?.next?.next;
  } while (slow !== fast);
    
  var h = head;
  var g = slow;
  while (h !== g) {
    h = h?.next;
    g = g?.next;
  }
  return h ?? null;
};
