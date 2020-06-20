/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
var addTwoNumbers = function(l1, l2) {
  var result = new ListNode(-1, null);
  var head = result;
  var moveup = 0;
  while (l1 !== null || l2 !== null || moveup >= 1) {
    var l1val = l1 ? l1.val : 0;
    var l2val = l2 ? l2.val : 0;
    var sum = l1val + l2val + moveup;
    moveup = Math.floor(sum / 10);
    if (moveup >= 1) {
      sum = Math.floor(sum % 10);
    }
    var node = new ListNode(sum);
    head.next = node;
    head = head.next;
    l1 = l1 ? l1.next : l1;
    l2 = l2 ? l2.next : l2;
  }
  return result.next;
};
