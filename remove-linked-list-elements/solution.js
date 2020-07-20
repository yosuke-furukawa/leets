/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @param {number} val
 * @return {ListNode}
 */
var removeElements = function(head, val) {
  const h = new ListNode(-1, head);
  let slow = h;
  let fast = slow.next;
  while(slow != null || fast != null) {
    if (slow.val === val) {
      slow = fast?.next;
    }
    if (fast?.val === val) {
      slow.next = fast?.next;
      fast = fast?.next;
    } else {
      slow = slow?.next;
      fast = fast?.next;      
    }
  }
  
  return h.next;
};
