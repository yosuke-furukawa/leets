/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var oddEvenList = function(head) {
  if (head === null) {
    return null;
  }
  var first = new ListNode(-1);
  var top = first;
  var second = new ListNode(-1);
  var secondTop = second;
  var odd = head;
  var even = head.next;
  while (odd !== null || even !== null) {
    var oddNode = odd ? new ListNode(odd.val) : null;
    var evenNode = even ? new ListNode(even.val) : null;
    first.next = oddNode;
    first = first.next;
    if (evenNode !== null) {
      second.next = evenNode;
      second = second.next;
    }
    if (odd.next === null) {
      break;
    }
    if (even === null || even.next === null) {
      break;
    }
    odd = odd.next.next;
    even = even.next.next;
  }
  first.next = secondTop.next;
  return top.next;
};
