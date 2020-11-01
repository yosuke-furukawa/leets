/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {number}
 */
var getDecimalValue = function(head) {
  
  var h = head;
  var num = 0;
  while (h != null) {
    num = (num << 1) | h.val;
    h = h.next;
  }
  return num;
};
